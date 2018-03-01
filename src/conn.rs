use std::io;
use std::process::Command;
use std::path::PathBuf;
use std::fs::File;
use std::io::Write;
use std::io::Read;
use xdg;
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
    let dirs = xdg_dirs().unwrap();
    let sock_path = dirs.place_config_file("conn.sock")
                        .expect("Could not create persistent socket file");

    let sock_str = sock_path.to_str().unwrap();
    let host_path = dirs.find_config_file(HOST_CONFIG_FILE);

    match host_path {
        Some(path) => ssh_command_with_host_path(path, sock_str, ssh_closure),
        None => Err(Error::NoConfigFile),
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

// Returns the XDG base dirs for slink
fn xdg_dirs() -> Result<xdg::BaseDirectories, xdg::BaseDirectoriesError> {
    xdg::BaseDirectories::with_prefix("slink")
}

// Run an ssh command, given the host path and socket string
fn ssh_command_with_host_path<F>(host_path: PathBuf, sock_str: &str, ssh_closure: F) -> SlinkResult<()>
    where  F: FnOnce(&mut Command) -> ()
{
    match File::open(host_path) {
        Ok(mut file) => {
            let mut host = String::new();
            match file.read_to_string(&mut host) {
                Ok(_) => (),
                Err(e) => return Err(Error::FailedConfigRead(e)),
            }
            ssh_command_with_host(host.as_str().trim(), sock_str, ssh_closure)
        },
        Err(e) => Err(Error::FailedConfigRead(e)),
    }
}

// Run an ssh command, given the actual host and the socket string
fn ssh_command_with_host<F>(host: &str, sock_str: &str, ssh_closure: F) -> SlinkResult<()>
    where  F: FnOnce(&mut Command) -> ()
{
    let proc_result = process::run("ssh", |cmd| {
        // "auto" ControlMaster setting means create a new connection if none
        // exists, and use the existing one if available
        cmd.arg("-oControlMaster=auto")
           // Use the passed-in socket string for the controlmaster path
           .arg(format!("-oControlPath={}", sock_str))
           // Hang onto the shared connection for 10mins after exit
           .arg("-oControlPersist=10m")
           // Force PTY allocation for interactivity
           .arg("-t")
           // And finally, SSH to the given host
           .arg(host);

        // Allow further configuration via the passed-in closure
        ssh_closure(cmd);
    });

    match proc_result {
        Ok(_) => Ok(()),
        Err(err) => Err(Error::ProcessError(err)),
    }
}
