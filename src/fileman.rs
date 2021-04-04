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

pub fn copy_files(file_names: Vec<String>, source: &str, dest: &str) -> std::io::Result<()> {
    if !Path::new(source).exists() {
        fs::create_dir(source).unwrap(); // create source dir if nonexistant
    }
    if !Path::new(dest).exists() {
        fs::create_dir(dest).unwrap(); // create destination dir if nonexistant
    }
    for file in file_names {
        let source_path = source.to_string() + &file;
        let dest_path = dest.to_string() + &file;
        // check if source file exists before attempting copy
        assert!(Path::new(&source_path).exists());
        fs::copy(source_path, dest_path)?;
    }
    Ok(())
}

pub fn remove_files(path: &str) {
    let files = fs::read_dir(path);
    for file in files {
        println!("{:?}", file);
    }
}



