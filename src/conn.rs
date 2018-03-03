use std::io;
use std::process::Command;
use std::fs::File;
use std::io::Write;
use std::io::Read;
use std::vec::Vec;
use std::path::PathBuf;
use xdg;
use isatty;
use process;
use errors::SlinkResult;

const HOST_CONFIG_FILE: &'static str = "hostname";

pub enum Error {
    NoConfigFile,
    FailedConfigWrite(io::Error),
    FailedConfigRead(io::Error),

    /*
     * SSH errors are all static strings; make it easier for consumers to use these
     * values by setting them to have the static lifetime
     */
    ProcessError(process::Error<'static>),
}

/*
 * Run an ssh command, passing the command as an argument to a closure for extra
 * configuration before running it
 */
pub fn ssh_command<F>(ssh_closure: F) -> SlinkResult<()>
    where  F: FnOnce(&mut Command) -> ()
{
    match get_host() {
        Err(e) => Err(e),
        Ok(host) => ssh_command_with_host(host.as_str(), ssh_closure),
    }
}

pub fn port_forward(ports: Vec<String>) -> SlinkResult<()> {
    match get_host() {
        Err(e) => Err(e),
        Ok(host) => {
            // Check for low ports, since those are privileged
            let mut has_low_port = false;
            let mut command = "ssh";
            let mut port_forwards: Vec<String> = Vec::new();
            for port in ports {
                if port.parse::<i32>().unwrap() < 1024 {
                    has_low_port = true;
                    command = "sudo";
                }
                port_forwards.push("-L".to_string());
                port_forwards.push(format!("{}:127.0.0.1:{}", port, port));
            }

            let proc_result = process::run(command, |cmd| {
                // If there's a low port, the command was just sudo. Actually
                // invoke ssh now.
                if has_low_port {
                    cmd.arg("ssh");
                }

                // Insert the options
                cmd.args(ssh_opts(host.as_str()));

                // Disable shell
                cmd.arg("-N");

                // Set up port forwards
                cmd.args(&port_forwards);

                // Using the remote host
                cmd.arg(host);
            });

            match proc_result {
                Ok(_) => Ok(()),
                Err(err) => Err(Error::ProcessError(err)),
            }
        }
    }
}

pub fn scp_up(from: PathBuf, to: PathBuf) -> SlinkResult<()> {
    match get_host() {
        Err(e) => Err(e),
        Ok(host) => scp(host.as_str(), |cmd| {
            cmd.arg(from.to_str().unwrap());
            cmd.arg(format!("{}:{}", host, to.to_str().unwrap()));
        }),
    }
}

/*
 * Set the host used for SSH connections.
 */
pub fn set_host(host: &str) -> SlinkResult<()> {
    let dirs = xdg_dirs().unwrap();
    let host_path = dirs.place_config_file(HOST_CONFIG_FILE)
                        .expect("Cannot create config file");

    let config_file = File::create(host_path);

    match config_file {
        Ok(mut file) => {
            match file.write(format!("{}\n", host).as_bytes()) {
                Ok(_) => Ok(()),
                Err(e) => Err(Error::FailedConfigWrite(e)),
            }
        },
        Err(e) => Err(Error::FailedConfigWrite(e)),
    }
}

/*
 * Get the host used for SSH connections.
 */
pub fn get_host() -> SlinkResult<String> {
    let dirs = xdg_dirs().unwrap();
    let host_path = dirs.find_config_file(HOST_CONFIG_FILE);

    match host_path {
        None => Err(Error::NoConfigFile),
        Some(path) => {
            match File::open(path) {
                Err(e) => Err(Error::FailedConfigRead(e)),
                Ok(mut file) => {
                    let mut host = String::new();
                    match file.read_to_string(&mut host) {
                        Ok(_) => (),
                        Err(e) => return Err(Error::FailedConfigRead(e)),
                    }
                    Ok(host.trim().to_string())
                },
            }
        },
    }
}

// Returns the XDG base dirs for slink
fn xdg_dirs() -> Result<xdg::BaseDirectories, xdg::BaseDirectoriesError> {
    xdg::BaseDirectories::with_prefix("slink")
}

pub fn ssh_opts(host: &str) -> Vec<String> {
    let dirs = xdg_dirs().unwrap();
    let sock_filename = format!("conn-{}.sock", host);
    let sock_path = dirs.place_cache_file(sock_filename)
                        .expect("Could not create persistent socket file");

    let sock_str = sock_path.to_str().unwrap();

    let mut vec = Vec::with_capacity(6);
    // "auto" ControlMaster setting means create a new connection if none
    // exists, and use the existing one if available
    vec.push(String::from("-oControlMaster=auto"));
    // Use the passed-in socket string for the controlmaster path
    vec.push(format!("-oControlPath={}", sock_str));
    // Hang onto the shared connection for 10mins after exit
    vec.push(String::from("-oControlPersist=10m"));

    vec
}

// Run an ssh command, given the actual host and the socket string
fn ssh_command_with_host<F>(host: &str, ssh_closure: F) -> SlinkResult<()>
    where  F: FnOnce(&mut Command) -> ()
{
    let proc_result = process::run("ssh", |cmd| {
        // Insert the options
        cmd.args(ssh_opts(host));

        // Force PTY allocation for interactivity if stdout is a tty
        if isatty::stdout_isatty() {
            cmd.arg("-t");
        }

        // Run in quiet mode
        cmd.arg("-q");

        // And finally, SSH to the given host
        cmd.arg(host);
        // Allow further configuration via the passed-in closure
        ssh_closure(cmd);
    });

    match proc_result {
        Ok(_) => Ok(()),
        Err(err) => Err(Error::ProcessError(err)),
    }
}

fn scp<F>(host: &str, closure: F) -> SlinkResult<()>
    where  F: FnOnce(&mut Command) -> ()
{
    let proc_result = process::run("scp", |cmd| {
        // Insert the options
        cmd.args(ssh_opts(host));
        // Allow further configuration via the passed-in closure
        closure(cmd);
    });

    match proc_result {
        Ok(_) => Ok(()),
        Err(err) => Err(Error::ProcessError(err)),
    }
}
