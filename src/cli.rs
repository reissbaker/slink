use std::path::PathBuf;
use std::vec::Vec;

#[derive(StructOpt, Debug)]
#[structopt(name = "slink", about = "Interact with remote machines over SSH")]
pub enum SlinkCommand {
    #[structopt(name = "use", about = "Update which remote machine slink uses")]
    Use {
        #[structopt(help = "The hostname of the remote machine")]
        host: String,
    },

    #[structopt(name = "go", about = "SSH to the remote")]
    Go {
        #[structopt(help = "The path to cd to on the remote; defaults to mirroring", parse(from_os_str))]
        path: Option<PathBuf>,
    },

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
        #[structopt(help = "Optional path on remote to upload to", short = "t", long = "to")]
        to: Option<String>,

        #[structopt(help = "Path to local file", parse(from_os_str))]
        path: PathBuf,
    },

    #[structopt(name = "download", about = "Download a file from the remote")]
    Download {
        #[structopt(help = "Optional path on remote to download from", short = "f", long = "from")]
        from: Option<String>,

        #[structopt(help = "Path to file", parse(from_os_str))]
        path: PathBuf,
    },

    #[structopt(name = "current", about = "Print current remote hostname")]
    Current,

    #[structopt(name = "forward", about = "Forward ports")]
    Forward {
        #[structopt(name = "PORTS", help = "Ports to forward")]
        ports: Vec<String>,
    },

    #[structopt(name = "debug", about = "Print various debug messages")]
    Debug,
}

#[derive(StructOpt, Debug)]
pub enum RsyncDirection {
    #[structopt(name = "up", about = "Sync directory up to the remote machine")]
    Up,

    #[structopt(name = "down", about = "Sync directory down from the remote machine")]
    Down,
}
