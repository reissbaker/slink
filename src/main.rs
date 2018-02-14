#[macro_use]
extern crate structopt;

use structopt::StructOpt;
use std::path::PathBuf;

#[derive(StructOpt, Debug)]
#[structopt(name = "slink", about = "Interact with remote machines over SSH")]
enum Slink {
    #[structopt(name = "use", about = "Update which remote machine slink uses")]
    Use {
        #[structopt(help = "The hostname of the remote machine")]
        host: String,
    },

    #[structopt(name = "go", about = "SSH to the remote")]
    Go,

    #[structopt(name = "run", about = "Run a command on the remote")]
    Run {
        #[structopt(help = "Command to run on the remote machine")]
        command: String,
    },

    #[structopt(name = "sync", about = "Sync to and from the remote")]
    Rsync {
        #[structopt(subcommand)]
        direction: RsyncDirection,
    },

    #[structopt(name = "upload", about = "Upload a file to the remote")]
    Upload {
        #[structopt(help = "Path to local file", parse(from_os_str))]
        path: PathBuf,
    },

    #[structopt(name = "download", about = "Download a file from the remote")]
    Download {
        #[structopt(help = "Path to remote file", parse(from_os_str))]
        path: PathBuf,
    },
}

#[derive(StructOpt, Debug)]
enum RsyncDirection {
    #[structopt(name = "up", about = "Sync directory up to the remote machine")]
    Up,

    #[structopt(name = "down", about = "Sync directory down from the remote machine")]
    Down,
}

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
