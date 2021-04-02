// all file manipulation goes here

extern crate toml;
extern crate serde;

use serde::{Deserialize};
use std::fs;

static APP_FILE: &'static str = "Applications.toml";

#[derive(Deserialize)]
pub struct Application {
    name: String,
    config_path: String,
    file_names: Vec<String>,
}

pub fn get_apps() -> Vec<Application> {
    let file_content = fs::read_to_string(APP_FILE).unwrap();
    let parsed: Vec<Application> = toml::from_str(&file_content).unwrap();
    parsed
}

pub fn copy_files(source: &str, dest: &str) -> std::io::Result<()> {
    fs::copy(source, dest)?;
    Ok(())
}

pub fn remove_files(path: &str) {
    let files = fs::read_dir(path);
    for file in files {
        println!("{:?}", file);
    }
}
