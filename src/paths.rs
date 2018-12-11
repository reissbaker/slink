use std::env;
use std::path::PathBuf;
use pathdiff;
use dirs;

pub fn same_path() -> PathBuf {
    match relative_pwd() {
        Some(relative_path) => relative_path,
        None => pwd_or_panic(),
    }
}

pub fn relative_pwd() -> Option<PathBuf> {
    dirs::home_dir().and_then(|home_dir| {
        let pwd = pwd_or_panic();
        pathdiff::diff_paths(&pwd, &home_dir)
    })
}

pub fn pwd_or_panic() -> PathBuf {
    env::current_dir().unwrap()
}
