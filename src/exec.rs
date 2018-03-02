use std::borrow::Cow;
use isatty;
use shell_escape;
use paths::{relative_pwd, pwd_or_panic};

pub fn command_in_same_path(command: &str) -> String {
    match relative_pwd() {
        Some(relative_path) => {
            let rel_str = relative_path.to_str().unwrap();
            command_in(rel_str, command)
        },
        None => {
            let pwd = pwd_or_panic();
            let pwd_str = pwd.to_str().unwrap();
            command_in(pwd_str, command)
        }
    }
}

pub fn shell_in_same_path() -> String {
    command_in_same_path("$SHELL --login")
}

pub fn command_in(path: &str, command: &str) -> String {
    let escaped = shell_escape::escape(Cow::Borrowed(path));

    // Escape the echo message separately, since otherwise you'd need to encase
    // in quotes (which would break shell escaping)
    let echo_string = format!("Running in remote directory: {}", path);
    let escaped_echo = shell_escape::escape(Cow::Borrowed(echo_string.as_str()));

    format!(
        "test -d {} {} && cd {} ; exec {}",
        escaped,
        // Log a UI message about the directory assuming stdout is a tty
        if isatty::stdout_isatty() {
            format!("&& echo {}", escaped_echo)
        } else {
            String::new()
        },
        escaped,
        command
    )
}
