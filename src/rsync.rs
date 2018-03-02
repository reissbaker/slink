use std::path::PathBuf;
use errors::SlinkResult;
use process;
use conn;

pub fn up(to: PathBuf) -> SlinkResult<()> {
    let host = match conn::get_host() {
        Ok(host) => host,
        Err(e) => return Err(e),
    };

    let result = process::run("rsync", |cmd| {
        // archive mode: preserve most things, allows modification-based optimizations
        cmd.arg("-a");
        // run quietly
        cmd.arg("-q");

        // use the persistent connection!
        cmd.arg("-e");
        cmd.arg(conn::ssh_opts(host.as_str()).join(" "));

        // Use the current directory
        cmd.arg(".");

        // finally, the host:dest string
        cmd.arg(format!("{}:{}", host, to.to_str().unwrap()));
    });

    match result {
        Ok(_) => Ok(()),
        Err(e) => Err(conn::Error::ProcessError(e)),
    }
}
