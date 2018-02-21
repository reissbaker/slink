use std::process::Command;

pub enum Error<'a> {
    FailedToLaunch(&'a str),
    FailedToWait(&'a str),
    NonZeroExit(&'a str, i32),
    KilledBySignal(&'a str),
}

/*
 * Run a configured command as a child process, block until completion, and
 * handle errors
 */
pub fn run<'a, F>(cmd_str: &'a str, cmd_closure: F) -> Result<(), Error<'a>>
    where F: FnOnce(&mut Command) -> ()
{
    // Build command and configure
    let mut command = Command::new(cmd_str);
    cmd_closure(&mut command);

    // Run and handle errors
    let mut child = match command.spawn() {
        Ok(child) => child,
        Err(_) => return Err(Error::FailedToLaunch(cmd_str)),
    };

    let exit_status = match child.wait() {
        Ok(status) => status,
        Err(_) => return Err(Error::FailedToWait(cmd_str)),
    };

    if exit_status.success() {
        return Ok(());
    }

    match exit_status.code() {
        Some(code) => Err(Error::NonZeroExit(cmd_str, code)),
        None => Err(Error::KilledBySignal(cmd_str)),
    }
}
