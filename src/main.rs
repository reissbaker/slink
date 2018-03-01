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

use structopt::StructOpt;
use std::env;
use std::path::PathBuf;
use std::borrow::Cow;
use cli::{SlinkCommand, RsyncDirection};
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
    conn::set_host(host.as_str())
}

fn go() -> SlinkResult<()> {
    conn::ssh_command(|ssh| {
        ssh.arg(exec_shell_in_same_path());
    })
}

fn exec_command_in_same_path(command: &str) -> String {
    let home_dir = env::home_dir().unwrap();
    let pwd = env::current_dir().unwrap();
    match pathdiff::diff_paths(&pwd, &home_dir) {
        Some(relative_path) => {
            let rel_str = relative_path.to_str().unwrap();
            exec_command_in(rel_str, command)
        },
        None => {
            let pwd_str = pwd.to_str().unwrap();
            exec_command_in(pwd_str, command)
        }
    }
}

fn exec_shell_in_same_path() -> String {
    exec_command_in_same_path("$SHELL --login")
}

fn exec_command_in(path: &str, command: &str) -> String {
    let escaped = shell_escape::escape(Cow::Borrowed(path));

    // Escape the echo message separately, since otherwise you'd need to encase
    // in quotes (which would break shell escaping)
    let echo_string = format!("Running in remote directory: {}", path);
    let escaped_echo = shell_escape::escape(Cow::Borrowed(echo_string.as_str()));

    format!(
        "test -d {} {} && cd {} ; exec {}",
        escaped,
        if isatty::stdout_isatty() {
            format!("&& echo {}", escaped_echo)
        } else {
            String::new()
        },
        escaped,
        command
    )
}

fn run(command: String) -> SlinkResult<()> {
    conn::ssh_command(|ssh| {
        ssh.arg(exec_command_in_same_path(command.as_str()));
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
