use std::io;
use std::fs::File;
use std::io::Write;
use std::io::Read;
use errors::SlinkResult;

use xdg;

const HOST_CONFIG_FILE: &'static str = "hostname";

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

// Returns the XDG base dirs for slink
pub fn xdg_dirs() -> Result<xdg::BaseDirectories, xdg::BaseDirectoriesError> {
    xdg::BaseDirectories::with_prefix("slink")
}
