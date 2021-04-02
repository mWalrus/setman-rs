// all file manipulation goes here

extern crate toml;
extern crate serde;

use serde::Deserialize;
use std::fs;

static APP_FILE: &'static str = "./Applications.toml";

#[derive(Deserialize)]
pub struct Apps {
    pub apps: Vec<Application>,
}

#[derive(Deserialize)]
pub struct Application {
    pub name: String,
    pub config_path: String,
    //pub file_names: Vec<String>,
}

impl Apps {
    pub fn new() -> Apps {
        Apps {
            apps: Vec::new()
        }
    }

    pub fn find_app_by_name(&mut self, app_name: &str) -> &Application {
        self.get_apps();
        let pos = self.apps.iter().position(|i| i.name == app_name).unwrap();
        let app = self.apps.get(pos).unwrap();
        app
    }

    pub fn get_apps(&mut self) {
        let file_content: String = fs::read_to_string(APP_FILE).unwrap();
        println!("{}", &file_content);
        self.apps = toml::from_str(&file_content).unwrap();
    }

    pub fn copy_files(self, source: &str, dest: &str) -> std::io::Result<()> {
        fs::copy(source, dest)?;
        Ok(())
    }

    pub fn remove_files(self, path: &str) {
        let files = fs::read_dir(path);
        for file in files {
            println!("{:?}", file);
        }
    }
}



