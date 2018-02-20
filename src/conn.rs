use std::process::Command;
use errors::SlinkResult;

pub enum ProcessError<'a> {
    FailedToLaunch(&'a str),
    FailedToWait(&'a str),
    NonZeroExit(&'a str, i32),
    KilledBySignal(&'a str),
}

// Run an ssh command, passing the command as an argument to a closure for extra
// configuration before running it
pub fn ssh_command<'a, F>(ssh_closure: F) -> SlinkResult<'a, ()>
    where  F: FnOnce(&mut Command) -> ()
{
    run_process("ssh", |cmd| {
        cmd.arg("shoebox");
        ssh_closure(cmd);
    })
}

// Run a configured command as a child process, block until completion, and
// handle errors
fn run_process<'a, F>(cmd_str: &'a str, cmd_closure: F) -> Result<(), ProcessError<'a>>
    where F: FnOnce(&mut Command) -> ()
{
    // Build command and configure
    let mut command = Command::new(cmd_str);
    cmd_closure(&mut command);

    // Run and handle errors
    let mut child = match command.spawn() {
        Ok(child) => child,
        Err(_) => return Err(ProcessError::FailedToLaunch(cmd_str)),
    };

    let exit_status = match child.wait() {
        Ok(status) => status,
        Err(_) => return Err(ProcessError::FailedToWait(cmd_str)),
    };

    if exit_status.success() {
        return Ok(());
    }

    match exit_status.code() {
        Some(code) => Err(ProcessError::NonZeroExit(cmd_str, code)),
        None => Err(ProcessError::KilledBySignal(cmd_str)),
    }
}
