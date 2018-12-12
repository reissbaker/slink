#[macro_use]
extern crate structopt;
extern crate xdg;
extern crate pathdiff;
extern crate shell_escape;
extern crate isatty;
extern crate dirs;

mod cli;
mod conn;
mod errors;
mod process;
mod paths;
mod exec;
mod rsync;
mod config;

use structopt::StructOpt;
use std::path::PathBuf;
use std::vec::Vec;
use cli::{SlinkCommand, RsyncDirection};
use errors::SlinkResult;

fn main() {
    let result = match SlinkCommand::from_args() {
        SlinkCommand::Use { host } => use_host(host),
        SlinkCommand::Current => current(),
        SlinkCommand::Go { path } => {
            match path {
                Some(path) => go(path),
                None => go(paths::same_path()),
            }
        }
        SlinkCommand::Run { command } => run(command),
        SlinkCommand::Forward { ports } => forward(ports),
        SlinkCommand::Rsync { direction } => {
            match direction {
                RsyncDirection::Up => rsync_up(),
                RsyncDirection::Down => rsync_down(),
            }
        },
        SlinkCommand::Upload { to, path } => upload(path, to),
        SlinkCommand::Download { path } => download(path),
        SlinkCommand::Debug => debug(),
    };

    match result {
        Ok(_) => {},
        Err(e) => errors::log_error_and_exit(e),
    };
}

fn use_host(host: String) -> SlinkResult<()> {
    println!("Using host: {}", host);
    config::set_host(host.as_str())
}

fn current() -> SlinkResult<()> {
    println!("{}", try!(config::get_host()));
    Ok(())
}

fn go(path: PathBuf) -> SlinkResult<()> {
    conn::ssh_command(|ssh| {
        ssh.arg(exec::shell_in(path));
    })
}

fn run(command: String) -> SlinkResult<()> {
    conn::ssh_command(|ssh| {
        ssh.arg(exec::command_in(paths::same_path(), command.as_str()));
    })
}

fn forward(ports: Vec<String>) -> SlinkResult<()> {
    println!("Forwarding {}...", ports.join(", "));
    println!("Leave this running to keep the ports forwarded.");
    println!("<Ctrl-C to exit>");
    conn::port_forward(ports)
}

fn rsync_up() -> SlinkResult<()> {
    rsync::up(paths::same_path())
}

fn rsync_down() -> SlinkResult<()> {
    rsync::down(paths::same_path())
}

fn upload(path: PathBuf, to: Option<String>) -> SlinkResult<()> {
    let to = to.map(|string| {
        let mut buf = PathBuf::new();
        buf.push(string);
        buf
    }).unwrap_or_else(|| {
        paths::same_path().join(path.canonicalize().unwrap().as_path())
    });

    conn::scp_up(path, to)
}

fn download(path: PathBuf) -> SlinkResult<()> {
    let from = paths::same_path().join(path.as_path());
    conn::scp_down(from, path)
}

fn debug() -> SlinkResult<()> {
    let ignored = config::ignored_files();
    println!("ignored files: {:?}", ignored);
    Ok(())
}
