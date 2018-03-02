use std::process::Command;
use std::path::PathBuf;
use errors::SlinkResult;
use process;
use conn;

pub fn up(to: PathBuf) -> SlinkResult<()> {
    let host = match conn::get_host() {
        Ok(host) => host,
        Err(e) => return Err(e),
    };

    rsync(host.as_str(), |cmd| {
        // Use the current directory
        cmd.arg(".");

        // finally, the host:dest string
        cmd.arg(format!("{}:{}", host, to.to_str().unwrap()));
    })
}

pub fn down(from: PathBuf) -> SlinkResult<()> {
    let host = match conn::get_host() {
        Ok(host) => host,
        Err(e) => return Err(e),
    };

    rsync(host.as_str(), |cmd| {
        // the host:dest string
        cmd.arg(format!("{}:{}/**", host, from.to_str().unwrap()));

        // write to the current directory
        cmd.arg(".");
    })
}

fn rsync<F>(host: &str, closure: F) -> SlinkResult<()>
    where  F: FnOnce(&mut Command) -> ()
{
    let result = process::run("rsync", |cmd| {
        // archive mode: preserve most things, allows modification-based optimizations
        cmd.arg("-a");
        cmd.arg("-v");

        // use the persistent connection!
        cmd.arg("-e");
        let ssh_opts_str = conn::ssh_opts(host).join(" ");
        cmd.arg(format!("ssh {}", ssh_opts_str));

        closure(cmd);
    });

    match result {
        Ok(_) => Ok(()),
        Err(e) => Err(conn::Error::ProcessError(e)),
    }
}
