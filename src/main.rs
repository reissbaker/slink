#[macro_use]
extern crate structopt;

mod cli;

use structopt::StructOpt;
use std::path::PathBuf;
use cli::{Slink, RsyncDirection};

fn main() {
    match Slink::from_args() {
        Slink::Use { host } => use_host(host),
        Slink::Go => go(),
        Slink::Run { command } => run(command),
        Slink::Rsync { direction } => {
            match direction {
                RsyncDirection::Up => rsync_up(),
                RsyncDirection::Down => rsync_down(),
            }
        },
        Slink::Upload { path } => upload(path),
        Slink::Download { path } => download(path),
    };
}

fn use_host(host: String) {
    println!("Using host: {}", host);
}

fn go() {
    println!("hello from go");
}

fn run(command: String) {
    println!("running: {}", command);
}

fn rsync_up() {
    println!("hello from up");
}

fn rsync_down() {
    println!("hello from down");
}

fn upload(path: PathBuf) {
    println!("uploading: {:?}", path);
}

fn download(path: PathBuf) {
    println!("downloading: {:?}", path);
}
