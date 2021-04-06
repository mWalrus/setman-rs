extern crate toml;
extern crate serde;

#[path = "./logger.rs"]
mod logger;

use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde()]
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
        Apps {
            items: Vec::new()
        }
    }

    pub fn find_app_by_name<'a>(&'a mut self, app_name: &str) -> App {
        self.read_apps();
        let pos: usize = self.items.iter().position(|i| i.name == app_name).unwrap();
        let app = self.items.get(pos).unwrap();
        app.clone()
    }

    pub fn read_apps(&mut self) {
        let mut file_content: String = fs::read_to_string("Applications.toml").unwrap();
        if file_content.len() == 1 {
            file_content = "items = []".to_string();
        }

        *self = toml::from_str::<Apps>(&file_content).unwrap();
    }

    pub fn save_new_app(&mut self, app_info: (String, String, Vec<String>)) {
        let (name, config_path, file_names) = app_info;
        let app: App = App::new(name, config_path, file_names);
        self.read_apps();
        self.items.push(app);
        self.write_toml();
    }

    pub fn remove_app(&mut self, app_name: &str) {
        self.read_apps();
        self.items = self.items.clone().into_iter().filter(|a| a.name.ne(app_name)).collect();
        self.write_toml();
    }

    fn write_toml(&self) {
        let toml = toml::to_vec(&self).unwrap();
        println!("{:?}", toml);
        fs::write("Applications.toml", &toml).unwrap();
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
        logger::print_warn("Removing file ".to_owned() + &file_path.display().to_string());
        fs::remove_file(&file_path).unwrap();
    }
}



