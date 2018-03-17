use process;
use config;

pub type SlinkResult<T> = Result<T, SlinkError>;
pub enum SlinkError {
    /*
     * SSH errors are all static strings; make it easier for consumers to use these
     * values by setting them to have the static lifetime
     */
    ProcessError(process::Error<'static>),
    ConfigError(config::Error),
}

impl From<process::Error<'static>> for SlinkError {
    fn from(e: process::Error<'static>) -> SlinkError {
        SlinkError::ProcessError(e)
    }
}

impl From<config::Error>for SlinkError {
    fn from(e: config::Error) -> SlinkError {
        SlinkError::ConfigError(e)
    }
}

pub fn log_error_and_exit(err: SlinkError) {
    println!("Slink encountered a fatal error:");

    // Return the exit code from match, which makes the type system enforce you
    // don't forget to have an exit (rather than trying to remember to always
    // call exit() in the match arms).
    let exit = match err {
        SlinkError::ProcessError(proc_err) => {
            match proc_err {
                process::Error::FailedToLaunch(name) => {
                    println!("Failed to launch {}", name);
                    2
                },

                process::Error::FailedToWait(name) => {
                    println!("Couldn't wait for {}", name);
                    3
                },

                process::Error::NonZeroExit(name, code) => {
                    println!("{} exited with code {}", name, code);
                    4
                },

                process::Error::KilledBySignal(name) => {
                    println!("{} killed by signal", name);
                    5
                },
            }
        },
        SlinkError::ConfigError(e) => {
            match e {
                config::Error::NoConfigFile => {
                    println!("No config file found; run slink use <host> to set up");
                    6
                },
                config::Error::FailedConfigWrite(e) => {
                    println!("Failed to write config file:");
                    println!("{}", e);
                    7
                },
                config::Error::FailedConfigRead(e) => {
                    println!("Failed to read config file:");
                    println!("{}", e);
                    8
                },
            }
        },
    };

    // TODO: panic for developers; exit nonzero for users
    panic!("Someday will exit with code {}", exit)
}
