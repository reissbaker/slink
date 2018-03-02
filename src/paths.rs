use std::env;
use std::path::PathBuf;
use pathdiff;

pub fn relative_pwd() -> Option<PathBuf> {
    match env::home_dir() {
        Some(home_dir) => {
            let pwd = pwd_or_panic();
            pathdiff::diff_paths(&pwd, &home_dir)
        },
        None => None,
    }
}

pub fn pwd_or_panic() -> PathBuf {
    env::current_dir().unwrap()
}
