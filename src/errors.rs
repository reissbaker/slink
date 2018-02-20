use conn::ProcessError;

pub type SlinkResult<'a, T> = Result<T, SlinkError<'a>>;
pub type SlinkError<'a> = ProcessError<'a>;

pub fn log_error_and_exit(err: SlinkError) {
    println!("Slink encountered a fatal error:");

    match err {
        ProcessError::FailedToLaunch(name) => println!("Failed to launch {}", name),
        ProcessError::FailedToWait(name) => println!("Couldn't wait for {}", name),
        ProcessError::NonZeroExit(name, code) => println!("{} exited with code {}", name, code),
        ProcessError::KilledBySignal(name) => println!("{} killed by signal", name),
    }

    // TODO: panic for developers; exit nonzero for users
    panic!()
}
