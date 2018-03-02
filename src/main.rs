#[macro_use]
extern crate structopt;
extern crate xdg;
extern crate pathdiff;
extern crate shell_escape;
extern crate isatty;

mod cli;
mod conn;
mod errors;
mod process;
mod paths;
mod exec;

use structopt::StructOpt;
use std::path::PathBuf;
use cli::{SlinkCommand, RsyncDirection};
use errors::SlinkResult;

fn main() {
    let result = match SlinkCommand::from_args() {
        SlinkCommand::Use { host } => use_host(host),
        SlinkCommand::Current => current(),
        SlinkCommand::Go => go(),
        SlinkCommand::Run { command } => run(command),
        SlinkCommand::Rsync { direction } => {
            match direction {
                RsyncDirection::Up => rsync_up(),
                RsyncDirection::Down => rsync_down(),
            }
        },
        SlinkCommand::Upload { path } => upload(path),
        SlinkCommand::Download { path } => download(path),
    };

    match result {
        Ok(_) => {},
        Err(e) => errors::log_error_and_exit(e),
    };
}

fn use_host(host: String) -> SlinkResult<()> {
    println!("Using host: {}", host);
    conn::set_host(host.as_str())
}

fn current() -> SlinkResult<()> {
    match conn::get_host() {
        Err(e) => Err(e),
        Ok(host) => {
            println!("{}", host);
            Ok(())
        },
    }
}

fn go() -> SlinkResult<()> {
    conn::ssh_command(|ssh| {
        ssh.arg(exec::shell_in_same_path());
    })
}

fn run(command: String) -> SlinkResult<()> {
    conn::ssh_command(|ssh| {
        ssh.arg(exec::command_in_same_path(command.as_str()));
    })
}

fn rsync_up() -> SlinkResult<()> {
    println!("hello from up");
    Ok(())
}

fn rsync_down() -> SlinkResult<()> {
    println!("hello from down");
    Ok(())
}

fn upload(path: PathBuf) -> SlinkResult<()> {
    println!("uploading: {:?}", path);
    Ok(())
}

fn download(path: PathBuf) -> SlinkResult<()> {
    println!("downloading: {:?}", path);
    Ok(())
}
