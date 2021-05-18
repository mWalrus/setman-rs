// Copyright (c) 2021 Oskar Hellkvist <hellkvistoskar@protonmail.com>

// SPDX-License-Identifier: BSD-2-Clause

use crate::colored;
use crate::logger;
use crate::paths;

use colored::*;
use paths::Paths;
use serde::{Deserialize, Serialize};
use std::{fs, path::PathBuf};
use std::process::exit;
use std::{io::Result, path::Path};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Apps {
    pub items: Vec<App>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct App {
    pub name: String,
    pub config_path: PathBuf,
    pub file_names: Vec<String>,
}

impl App {
    pub fn new(name: String, config_path: String, file_names: Vec<String>) -> App {
        let config_path = Paths::new().get_absolute_path(&config_path);
        App {
            name,
            config_path,
            file_names,
        }
    }
}

impl Apps {
    pub fn new() -> Apps {
        let file_content: String = match fs::read_to_string(Paths::new().applist_path) {
            Ok(content) => content,
            Err(e) => {
                println!("Error opening file: {}", e);
                exit(0);
            }
        };

        match toml::from_str::<Apps>(&file_content) {
            Ok(toml) => toml,
            Err(_e) => toml::from_str::<Apps>("items = []").unwrap(),
        }
    }

    pub fn find_app_by_name<'a>(&'a mut self, app_name: &str) -> Option<App> {
        let pos: usize = match self.items.iter().position(|i| i.name == app_name) {
            Some(pos) => pos,
            None => {
                logger::print_warn(format!(
                    "Application with name '{}' could not be found",
                    &app_name
                ));
                exit(0);
            }
        };
        Some(self.items.get(pos)?.clone())
    }

    pub fn save_new_app(&mut self, app: App) -> Result<()> {
        for app_item in self.items.clone() {
            if app_item.name.eq(&app.name) {
                logger::print_warn("An app with that name already exists".to_string());
                return Ok(());
            }
        }
        self.items.push(app);
        self.write_toml()?;
        Ok(())
    }

    pub fn remove_app(&mut self, app_name: &str) -> Result<()> {
        self.items.retain(|a| a.name.ne(app_name));
        self.write_toml()?;
        Ok(())
    }

    fn write_toml(&self) -> Result<()>{
        let toml = toml::to_string(&self).unwrap();
        fs::write(Paths::new().applist_path, &toml)?;
        Ok(())
    }
}

pub fn get_dir_names_in_path(dir_path: &PathBuf) -> Result<Vec<String>> {
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

pub fn copy_files(file_names: Vec<String>, source: &PathBuf, dest: &PathBuf) -> Result<()> {
    logger::print_job(format!("Copying files from {:#?}", source));
    assert!(source.exists());
    assert!(dest.exists());
    for file in file_names {
        let mut source_path = source.clone();
        source_path.set_file_name(&file);
        let mut dest_path = dest.clone();
        dest_path.set_file_name(&file);
        // check if source file exists before attempting copy
        assert!(Path::new(&source_path).exists());
        fs::copy(source_path, dest_path)?;
        logger::print_info(format!("Copied {} to {:#?}", &file.bold(), &dest));
    }
    Ok(())
}

pub fn remove_files(conf_path: &PathBuf) -> Result<()> {
    logger::print_job(format!("Removing files in {:#?}", &conf_path));
    let files = fs::read_dir(conf_path)?;
    for file in files {
        let file_path = file?.path();
        logger::print_info(
            format!("Removing file {}", &file_path.display().to_string())
        );
        fs::remove_file(&file_path)?;
    }
    Ok(())
}
