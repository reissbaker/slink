use conn::{ProcessError, SshError};

pub type SlinkResult<T> = Result<T, SlinkError>;
pub type SlinkError = SshError;

pub fn log_error_and_exit(err: SlinkError) {
    println!("Slink encountered a fatal error:");

    // Return the exit code from match, which makes the type system enforce you
    // don't forget to have an exit (rather than trying to remember to always
    // call exit() in the match arms).
    let exit = match err {
        ProcessError::FailedToLaunch(name) => {
            println!("Failed to launch {}", name);
            2
        },

        ProcessError::FailedToWait(name) => {
            println!("Couldn't wait for {}", name);
            3
        },

        ProcessError::NonZeroExit(name, code) => {
            println!("{} exited with code {}", name, code);
            4
        },

        ProcessError::KilledBySignal(name) => {
            println!("{} killed by signal", name);
            5
        },
    };

    // TODO: panic for developers; exit nonzero for users
    panic!("Someday will exit with code {}", exit)
}
