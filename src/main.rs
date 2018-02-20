#[macro_use]
extern crate structopt;

mod cli;
mod conn;
mod errors;

use structopt::StructOpt;
use std::path::PathBuf;
use cli::{SlinkCommand, RsyncDirection};
use conn::ssh_command;
use errors::SlinkResult;

fn main() {
    let result = match SlinkCommand::from_args() {
        SlinkCommand::Use { host } => use_host(host),
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
    Ok(())
}

fn go() -> SlinkResult<()> {
    println!("hello from go");
    ssh_command(|_| {})
}

fn run(command: String) -> SlinkResult<()> {
    println!("running: {}", command);
    ssh_command(|ssh| {
        ssh.arg(command);
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
