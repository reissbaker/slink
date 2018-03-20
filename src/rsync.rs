use std::process::Command;
use std::path::PathBuf;
use errors::SlinkResult;
use process;
use conn;
use config;
use paths;

use pathdiff;

pub fn up(to: PathBuf) -> SlinkResult<()> {
    let host = try!(config::get_host());
    let ignored = config::ignored_files();

    rsync(host.as_str(), |cmd| {
        // Use the current directory
        cmd.arg(".");

        // Check all the ignores
        for ignore in ignored.iter() {
            // rsync expects all file paths to be relative. If you're looking at
            // an absolute path, figure out if it's a subdirectory of pwd, and
            // if so, pass the relative path to rsync's --exclude
            if ignore.starts_with("/") {
                let maybe_rel_path = pathdiff::diff_paths(
                    &PathBuf::from(ignore),
                    &paths::pwd_or_panic()
                );
                match maybe_rel_path {
                    None => (),
                    Some(rel_path) => {
                        if !rel_path.starts_with("../") {
                            ignore_path(cmd, rel_path);
                        }
                    },
                }
            }
            // Otherwise if it's relative, just pass it straight through
            else {
                ignore_path(cmd, PathBuf::from(ignore));
            }
        }

        // finally, the host:dest string
        cmd.arg(format!("{}:{}", host, to.to_str().unwrap()));
    })
}

fn ignore_path(cmd: &mut Command, rel_path: PathBuf) {
    cmd.arg("--exclude");
    cmd.arg(rel_path);
}

pub fn down(from: PathBuf) -> SlinkResult<()> {
    let host = try!(config::get_host());

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
    try!(process::run("rsync", |cmd| {
        // archive mode: preserve most things, allows modification-based optimizations
        cmd.arg("-a");
        cmd.arg("-v");

        // Delete extraneous files
        cmd.arg("--delete");

        // use the persistent connection!
        cmd.arg("-e");
        let ssh_opts_str = conn::ssh_opts(host).join(" ");
        cmd.arg(format!("ssh {}", ssh_opts_str));

        closure(cmd);
    }));

    Ok(())
}
