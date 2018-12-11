use std::path::PathBuf;
use std::env;
use std::io;
use std::fs::File;
use std::io::Write;
use std::io::Read;
use std::collections::HashSet;
use errors::SlinkResult;
use paths::relative_pwd;

use xdg;

const HOST_CONFIG_FILE: &'static str = "hostname";
const HOST_ENV_VAR: &'static str = "SLINK_HOST";

pub enum Error {
    NoConfigFile,
    FailedConfigWrite(io::Error),
    FailedConfigRead(io::Error),
}

/*
 * Set the host used for SSH connections.
 */
pub fn set_host(host: &str) -> SlinkResult<()> {
    let dirs = xdg_dirs().unwrap();
    let host_path = dirs.place_config_file(HOST_CONFIG_FILE)
                        .expect("Cannot create config file");

    let mut file = try!(File::create(host_path).map_err(|e| {
        Error::FailedConfigWrite(e)
    }));

    try!(file.write(format!("{}\n", host).as_bytes()).map_err(|e| {
        Error::FailedConfigWrite(e)
    }));

    Ok(())
}

/*
 * Get the host used for SSH connections.
 */
pub fn get_host() -> SlinkResult<String> {
    match env::var(HOST_ENV_VAR) {
        Ok(val) => Ok(val),
        Err(_) => {
            let dirs = xdg_dirs().unwrap();
            let path = try!(
                dirs.find_config_file(HOST_CONFIG_FILE).ok_or(Error::NoConfigFile)
            );

            let mut file = try!(File::open(path).map_err(|e| {
                Error::FailedConfigRead(e)
            }));

            let mut host = String::new();
            try!(file.read_to_string(&mut host).map_err(|e| {
                Error::FailedConfigRead(e)
            }));

            Ok(host.trim().to_string())
        }
    }
}

// Returns the XDG base dirs for slink
pub fn xdg_dirs() -> Result<xdg::BaseDirectories, xdg::BaseDirectoriesError> {
    xdg::BaseDirectories::with_prefix("slink")
}

pub fn ignored_files() -> HashSet<String> {
    let dirs = xdg_dirs().unwrap();
    let mut ignored: HashSet<String> = HashSet::new();
    let home_dir = dirs::home_dir().unwrap();

    match dirs.find_config_file("ignore") {
        Some(path) => read_ignore_file(home_dir.clone(), path, &mut ignored),
        None => (),
    };

    let relative_pwd = relative_pwd().unwrap();
    let mut search_path: Option<PathBuf> = None;
    for component in relative_pwd.iter() {
        let current_search = search_path.map_or(PathBuf::from(component), |s| {
            s.join(component)
        });
        search_path = Some(current_search.clone());
        let ignored_file: PathBuf = current_search.clone().join(".slink/ignore");
        read_ignore_file(
            home_dir.clone().join(current_search),
            home_dir.join(ignored_file),
            &mut ignored
        );
    }

    ignored
}

fn read_ignore_file(root: PathBuf, path: PathBuf, ignored: &mut HashSet<String>) {
    let mut contents = String::new();
    match File::open(path.clone()) {
        Err(_) => (),
        Ok(mut file) => {
            match file.read_to_string(&mut contents) {
                Err(_) => (),
                Ok(_) => {
                    let lines = contents.split("\n");
                    for line in lines {
                        match parse_ignore(root.clone(), line) {
                            Some(ignore) => { ignored.insert(ignore); },
                            None => (),
                        }
                    }
                }
            }
        }
    };
}

fn parse_ignore(path: PathBuf, ignore: &str) -> Option<String> {
    if ignore == "" {
        return None;
    }
    if ignore.chars().peekable().peek() != Some(&'/') {
        return Some(String::from(ignore));
    }
    let mut chars = ignore.chars();
    chars.next();
    return Some(path.join(chars.as_str()).to_str().unwrap().into());
}
