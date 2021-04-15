extern crate toml;
extern crate serde;

#[path = "logger.rs"]
mod logger;
#[path = "paths.rs"]
mod paths;

use serde::{Deserialize, Serialize};
use std::path::Path;
use std::fs;
use std::process::exit;
use paths::Paths;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Apps {
    pub items: Vec<App>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct App {
    pub name: String,
    pub config_path: String,
    pub file_names: Vec<String>,
}

impl App {
    pub fn new(name: String, config_path: String, file_names: Vec<String>) -> App {
        App {
            name,
            config_path,
            file_names
        }
    }
}

impl Apps {
    pub fn new() -> Apps {
        let file_content: String = match fs::read_to_string(Paths::new().apps_config_path) {
            Ok(content) => content,
            Err(e) => {
                println!("Error opening file: {}", e);
                exit(0);
            }
        };

        match toml::from_str::<Apps>(&file_content) {
            Ok(toml) => toml,
            Err(e) => {
                println!("Failed to parse toml: {}", e);
                exit(0);
            }
        }
    }

    pub fn find_app_by_name<'a>(&'a mut self, app_name: &str) -> App {
        let pos: Option<usize> = self.items.iter().position(|i| i.name == app_name);
        if pos == None {
            logger::print_warn("Application with name '".to_owned() + &app_name + "' could not be found");
            exit(0);
        }
        let app = self.items.get(pos.unwrap()).unwrap();
        app.clone()
    }

    pub fn save_new_app(&mut self, app: App) {
        self.items.push(app);
        self.write_toml();
    }

    pub fn remove_app(&mut self, app_name: &str) {
        self.items = self.items.clone().into_iter().filter(|a| a.name.ne(app_name)).collect();
        self.write_toml();
    }

    fn write_toml(&self) {
        let toml = toml::to_string(&self).unwrap();
        fs::write(Paths::new().apps_config_path, &toml).unwrap();
    }
}

pub fn dir_exists(path: &str) {
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
        logger::print_warn("Removing file ".to_owned() + &file_path.display().to_string());
        fs::remove_file(&file_path).unwrap();
    }
}
