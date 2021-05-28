// Copyright (c) 2021 Oskar Hellkvist <hellkvistoskar@protonmail.com>

// SPDX-License-Identifier: BSD-2-Clause

use crate::colored;
use crate::logger;
use crate::paths;
use crate::regex;
use crate::thiserror;

use colored::*;
use paths::Paths;
use serde::{Deserialize, Serialize};
use std::{fs, path::PathBuf};
use std::{io::Result as IOResult, path::Path};
use regex::Regex;
use thiserror::Error;

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

#[derive(Error, Debug)]
pub enum AppError {
    #[error("An application with name '{0}' could not be found.")]
    NotFound(String),
    #[error("An application with that name already exists")]
    Duplicate,

}

#[derive(Error, Debug)]
pub enum TOMLError {
    #[error("Failed to read from file")]
    FileError {
        #[from]
        source: std::io::Error,
    },
    #[error("Failed to parse toml")]
    ParseError {
        #[from]
        source: toml::de::Error,
    }
}

impl App {
    pub fn new(name: String, config_path: String, file_names: Vec<String>) -> App {
        let config_path = paths::get_absolute_path(&config_path);
        App {
            name,
            config_path,
            file_names,
        }
    }
}

impl Apps {
    pub fn new() -> Apps {
        let file_content: String = match fs::read_to_string(
            Paths::default().applist_path
        ) {
            Ok(content) => Ok(content),
            Err(e) => Err(TOMLError::FileError{source: e}),
        }.unwrap();

        match toml::from_str::<Apps>(&file_content) {
            Ok(toml) => toml,
            Err(_e) => toml::from_str::<Apps>("items = []").unwrap(),
        }
    }

    pub fn find_app_by_name<'a>(&'a mut self, app_name: &str) -> Option<App> {
        let pos: usize = match self.items
                .iter()
                .position(|i| i.name == app_name) {
            Some(pos) => pos,
            None => {
                panic!("{}", AppError::NotFound(app_name.to_string()))
            }
        };
        Some(self.items.get(pos)?.clone())
    }

    pub fn find_apps_from_regex<'a>(&'a self, regex: &str) -> Option<Vec<&App>> {
        let re = Regex::new(regex).unwrap();
        let apps = self.items
            .iter()
            .filter(|app| re.is_match(&app.name))
            .collect();
        Some(apps)
    }

    pub fn save_new_app(&mut self, app: App) -> IOResult<()> {
        for app_item in self.items.clone() {
            if app_item.name.eq(&app.name) {
                panic!("{}", AppError::Duplicate);
            }
        }
        self.items.push(app);
        self.write_toml()?;
        Ok(())
    }

    pub fn remove_app(&mut self, app_name: &str) -> IOResult<()> {
        self.items.retain(|a| a.name.ne(app_name));
        self.write_toml()?;
        Ok(())
    }

    fn write_toml(&self) -> IOResult<()>{
        let toml = toml::to_string(&self).unwrap();
        fs::write(Paths::default().applist_path, &toml)?;
        Ok(())
    }
}

pub fn get_dir_names_in_path(dir_path: &PathBuf) -> IOResult<Vec<String>> {
    let read = Path::new(dir_path).read_dir()?;
    let mut result: Vec<String> = Vec::new();
    for e in read {
        let entry = e?;
        if entry.path().is_dir() {
            result.push(
                entry.file_name()
                    .to_str()
                    .unwrap()
                    .to_string()
            );
        }
    }
    Ok(result)
}

pub fn copy_files(
    file_names: Vec<String>,
    source: &PathBuf,
    dest: &PathBuf
) -> IOResult<()> {
    logger::print_job(&format!("Copying files from {:?}", source));
    assert!(source.exists());
    assert!(dest.exists());
    for file in file_names {
        let mut source_path = source.clone();
        source_path.push(&file);
        let mut dest_path = dest.clone();
        dest_path.push(&file);
        // check if source file exists before attempting copy
        assert!(source_path.exists());
        fs::copy(source_path, dest_path)?;
        logger::print_info(
            &format!("Copied {} to {:?}", &file.bold(), &dest)
        );
    }
    Ok(())
}

pub fn remove_files(conf_path: &PathBuf) -> IOResult<()> {
    logger::print_job(
        &format!("Removing files in {:#?}", &conf_path)
    );
    let files = fs::read_dir(conf_path)?;
    for file in files {
        let file_path = file?.path();
        logger::print_info(
            &format!("Removing file {:?}", &file_path)
        );
        fs::remove_file(&file_path)?;
    }
    Ok(())
}
