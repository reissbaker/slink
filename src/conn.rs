use std::process::Command;
use std::vec::Vec;
use std::path::PathBuf;
use std::convert;
use isatty;
use process;
use errors::SlinkResult;
use config::{get_host, xdg_dirs};

/*
 * Run an ssh command, passing the command as an argument to a closure for extra
 * configuration before running it
 */
pub fn ssh_command<F>(ssh_closure: F) -> SlinkResult<()>
    where  F: FnOnce(&mut Command) -> ()
{
    let host = try!(get_host());
    ssh_command_with_host(host.as_str(), ssh_closure)
}

pub fn port_forward(ports: Vec<String>) -> SlinkResult<()> {
    let host = try!(get_host());

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

    try!(process::run(command, |cmd| {
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
    }));

    Ok(())
}

pub fn scp_up(from: PathBuf, to: PathBuf) -> SlinkResult<()> {
    let host = try!(get_host());
    scp(host.as_str(), |cmd| {
        cmd.arg(from.to_str().unwrap());
        cmd.arg(format!("{}:{}", host, to.to_str().unwrap()));
    })
}

pub fn scp_down(from: PathBuf, to: PathBuf) -> SlinkResult<()> {
    let host = try!(get_host());
    scp(host.as_str(), |cmd| {
        cmd.arg(format!("{}:{}", host, from.to_str().unwrap()));
        cmd.arg(to.to_str().unwrap());
    })
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

        // 130 is 128+2, aka SIGINT. This appears to be generated sometimes when
        // you log out of the remote connection -- not sure why? But doesn't
        // appear to be a fatal error.
        Err(process::Error::NonZeroExit(_, 130)) => Ok(()),

        Err(e) => Err(convert::From::from(e)),
    }
}

fn scp<F>(host: &str, closure: F) -> SlinkResult<()>
    where  F: FnOnce(&mut Command) -> ()
{
    try!(process::run("scp", |cmd| {
        // Insert the options
        cmd.args(ssh_opts(host));
        // Allow further configuration via the passed-in closure
        closure(cmd);
    }));

    Ok(())
}
