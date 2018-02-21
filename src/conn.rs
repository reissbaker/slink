use std::process::Command;
use process;
use errors::SlinkResult;

/*
 * SSH errors are all static strings; make it easier for consumers to use these
 * values by setting them to have the static lifetime
 */
pub type Error = process::Error<'static>;

/*
 * Run an ssh command, passing the command as an argument to a closure for extra
 * configuration before running it
 */
pub fn ssh_command<F>(ssh_closure: F) -> SlinkResult<()>
    where  F: FnOnce(&mut Command) -> ()
{
    process::run("ssh", |cmd| {
        cmd.arg("shoebox");
        ssh_closure(cmd);
    })
}
