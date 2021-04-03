extern crate toml;
extern crate serde;

use serde::Deserialize;
use std::fs;

#[derive(Deserialize)]
pub struct Apps {
    pub apps: Vec<Application>,
}

#[derive(Deserialize)]
pub struct Application {
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

    pub fn find_app_by_name(&mut self, app_name: &str) -> &Application {
        self.get_apps();
        let pos: usize = self.apps.iter().position(|i| i.name == app_name).unwrap();
        self.apps.get(pos).unwrap()
    }

    pub fn get_apps(&mut self) {
        let file_content: String = fs::read_to_string("Applications.toml").unwrap();
        self.apps = toml::from_str::<Apps>(&file_content).unwrap().apps;
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



