#[macro_use]
extern crate structopt;

mod cli;

use std::io;
use std::io::{Error, ErrorKind};
use std::process::Command;
use structopt::StructOpt;
use std::path::PathBuf;
use cli::{SlinkCommand, RsyncDirection};

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
        Err(e) => {
            // TODO: panic for developers; exit nonzero for users
            // TODO: invent your own error hierarchy rather than using
            // io::Result, so that you can exit with meaningful exit codes
            panic!("Error: {}", e);
        },
    };
}

fn use_host(host: String) -> io::Result<()> {
    println!("Using host: {}", host);
    Ok(())
}

fn go() -> io::Result<()> {
    println!("hello from go");
    ssh_command(|_| {})
}

fn run(command: String) -> io::Result<()> {
    println!("running: {}", command);
    ssh_command(|ssh| {
        ssh.arg(command);
    })
}

fn rsync_up() -> io::Result<()> {
    println!("hello from up");
    Ok(())
}

fn rsync_down() -> io::Result<()> {
    println!("hello from down");
    Ok(())
}

fn upload(path: PathBuf) -> io::Result<()> {
    println!("uploading: {:?}", path);
    Ok(())
}

fn download(path: PathBuf) -> io::Result<()> {
    println!("downloading: {:?}", path);
    Ok(())
}

fn ssh_command<F>(ssh_closure: F) -> io::Result<()>
    where  F: FnOnce(&mut Command) -> ()
{
    let mut command = Command::new("ssh");
    command.arg("shoebox");

    // Allow configuration
    ssh_closure(&mut command);

    // Run and handle errors
    let mut child = try!(
        command.spawn()
    );
    let exit_status = try!(child.wait());
    if exit_status.success() {
        return Ok(());
    }

    let error = match exit_status.code() {
        Some(code) => Error::new(ErrorKind::Other, format!("SSH exited with exit code {}", code)),
        None => Error::new(ErrorKind::Interrupted, "SSH was killed by signal"),
    };

    Err(error)
}
