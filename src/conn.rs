use std::process::Command;
use errors::SlinkResult;

pub enum ProcessError {
    FailedToLaunch(&'static str),
    FailedToWait(&'static str),
    NonZeroExit(&'static str, i32),
    KilledBySignal(&'static str),
}

// Run an ssh command, passing the command as an argument to a closure for extra
// configuration before running it
pub fn ssh_command<F>(ssh_closure: F) -> SlinkResult<()>
    where  F: FnOnce(&mut Command) -> ()
{
    run_process("ssh", |cmd| {
        ssh_closure(cmd);
    })
}

// Run a configured command as a child process, block until completion, and
// handle errors
fn run_process<F>(cmd_str: &'static str, cmd_closure: F) -> Result<(), ProcessError>
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
