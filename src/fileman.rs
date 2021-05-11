// Copyright (c) 2021 Oskar Hellkvist <hellkvistoskar@protonmail.com>

// SPDX-License-Identifier: BSD-2-Clause

use crate::logger;
use crate::paths;
use crate::colored;

use serde::{Deserialize, Serialize};
use std::{io::Result, path::Path};
use std::fs;
use std::process::exit;
use paths::Paths;
use colored::*;

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
            Err(_e) => {
                toml::from_str::<Apps>("items = []").unwrap()
            }
        }
    }

    pub fn find_app_by_name<'a>(&'a mut self, app_name: &str) -> Option<App> {
        let pos: usize = match self.items.iter().position(|i| i.name == app_name) {
            Some(pos) => pos,
            None => {
                logger::print_warn(format!("Application with name '{}' could not be found", &app_name));
                exit(0);
            }
        };
        Some(self.items.get(pos)?.clone())
    }

    pub fn save_new_app(&mut self, app: App) {
        for app_item in self.items.clone() {
            if app_item.name.eq(&app.name) {
                logger::print_warn("An app with that name already exists".to_string());
                return
            }
        }
        self.items.push(app);
        self.write_toml();
    }

    pub fn remove_app(&mut self, app_name: &str) {
        self.items.retain(|a| a.name.ne(app_name));
        self.write_toml();
    }

    fn write_toml(&self) {
        let toml = toml::to_string(&self).unwrap();
        fs::write(Paths::new().apps_config_path, &toml).unwrap();
    }
}

pub fn path_exists(path: &str) {
    if !Path::new(path).exists() {
        logger::print_info("Creating ".to_owned() + path);
        fs::create_dir(path).unwrap(); // create dir if nonexistant
    };
}

pub fn get_dir_names_in_path(dir_path: &str) -> Result<Vec<String>> {
    let read = Path::new(dir_path).read_dir()?;
    let mut result: Vec<String> = Vec::new();
    for e in read {
        let entry = e?;
        if entry.path().is_dir() {
            result.push(entry.file_name().to_str().unwrap().to_string());
        }
    }
    Ok(result)
}

pub fn copy_files(file_names: Vec<String>, source: &str, dest: &str) -> Result<()> {
    logger::print_job(format!("Copying files from {}", source));
    path_exists(source);
    path_exists(dest);
    for file in file_names {
        let source_path = format!("{}/{}", &source, &file);
        let dest_path = format!("{}/{}", &dest, &file);
        // check if source file exists before attempting copy
        assert!(Path::new(&source_path).exists());
        fs::copy(source_path, dest_path)?;
        logger::print_info(format!("Copied {} to {}", &file.bold(), &dest));
    }
    Ok(())
}

pub fn remove_files(conf_path: &str) -> Result<()>{
    let files = fs::read_dir(conf_path)?;
    for file in files {
        let file_path = file?.path();
        logger::print_warn("Removing file ".to_owned() + &file_path.display().to_string());
        fs::remove_file(&file_path)?;
    }
    Ok(())
}
