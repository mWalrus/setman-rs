extern crate toml;
extern crate serde;

#[path = "./logger.rs"]
mod logger;

use serde::Deserialize;
use std::fs;
use std::path::Path;

#[derive(Deserialize, Clone)]
pub struct Apps {
    pub apps: Vec<App>,
}

#[derive(Deserialize, Clone)]
pub struct App {
    pub name: String,
    pub config_path: String,
    pub file_names: Vec<String>,
}

impl Apps {
    pub fn new() -> Apps {
        Apps {
            apps: Vec::new()
        }
    }

    pub fn find_app_by_name<'a>(&'a mut self, app_name: &str) -> App {
        self.read_apps();
        let pos: usize = self.apps.iter().position(|i| i.name == app_name).unwrap();
        let app = self.apps.get(pos).unwrap();
        app.clone()
    }

    pub fn read_apps(&mut self) {
        let file_content: String = fs::read_to_string("Applications.toml").unwrap();
        self.apps = toml::from_str::<Apps>(&file_content).unwrap().apps;
    }
}

fn dir_exists(path: &str) {
    if !Path::new(path).exists() {
        logger::print_warn("Couldn't find ".to_owned() + path);
        logger::print_info("Creating ".to_owned() + path);
        fs::create_dir(path).unwrap(); // create dir if nonexistant
    };
}

pub fn copy_files(file_names: Vec<String>, source: &str, dest: &str) -> std::io::Result<()> {
    dir_exists(source);
    dir_exists(dest);
    for file in file_names {
        let source_path = source.to_string() + &file;
        let dest_path = dest.to_string() + &file;
        // check if source file exists before attempting copy
        assert!(Path::new(&source_path).exists());
        fs::copy(source_path, dest_path)?;
    }
    Ok(())
}

pub fn remove_files(conf_path: &str) {
    let files = fs::read_dir(conf_path).unwrap();
    for file in files {
        let file_path = file.unwrap().path();
        logger::print_info("Removing file ".to_owned() + &file_path.display().to_string());
        fs::remove_file(&file_path).unwrap();
    }
}



