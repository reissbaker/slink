use std::path::PathBuf;
use std::borrow::Cow;
use isatty;
use shell_escape;

pub fn shell_in(path: PathBuf) -> String {
    command_in(path, "$SHELL --login")
}

pub fn command_in(path_buf: PathBuf, command: &str) -> String {
    let path = path_buf.to_str().unwrap();
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
