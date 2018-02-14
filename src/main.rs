#[macro_use]
extern crate structopt;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "slink", about = "Interact with remote machines over SSH")]
enum Slink {
    #[structopt(name = "go", about = "SSH to the remote")]
    Go {
    },
    #[structopt(name = "run", about = "Run a command on the remote")]
    Run {
        #[structopt(help = "Command to run on the remote machine")]
        command: String,
    },
    #[structopt(name = "up", about = "Sync directory up to the remote machine")]
    Up {
    },
    #[structopt(name = "down", about = "Sync directory down from the remote machine")]
    Down {
    },
}

fn main() {
    let opt = Slink::from_args();
    match opt {
        Slink::Go { } => go(),
        Slink::Run { command } => run(command),
        Slink::Up { } => up(),
        Slink::Down { } => down(),
    };
}

fn go() {
    println!("hello from go");
}

fn run(command: String) {
    println!("running: {}", command);
}

fn up() {
    println!("hello from up");
}

fn down() {
    println!("hello from down");
}
