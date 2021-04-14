extern crate git2;
extern crate uuid;
extern crate toml;
extern crate serde;

#[path = "readline.rs"]
mod readline;
#[path = "logger.rs"]
mod logger;

use git2::Repository;
use uuid::Uuid;
use std::fs;
use std::process::exit;
use serde::Deserialize;
use toml::Value;

static GIT_FILE: &str = "git.toml";

#[derive(Deserialize, Clone)]
pub struct GitRepo {
    upstream_url: String,
    repo_path: String,
}

// TODO: implement push functionality
impl GitRepo {
    pub fn new() -> GitRepo {
        let file_content = match fs::read_to_string(GIT_FILE) {
            Ok(content) => content,
            Err(_e) => {
                logger::print_warn(format!("File {} not found, exiting", GIT_FILE));
                exit(0);
            }
        };
        let upstream_url: String = match toml::from_str::<Value>(&file_content) {
            Ok(value) => value["upstream_url"].as_str().unwrap().to_string(),
            Err(_e) => {
                logger::print_warn("Upstream url not found, exiting".to_string());
                exit(0);
            }
        };
        let repo_name = "setman-tmp".to_string() + &Uuid::new_v4().to_string();
        let repo_path = "/tmp/".to_string() + &repo_name;
        GitRepo {
            upstream_url,
            repo_path,
        }
    }
    pub fn get_dir_names(self) -> Vec<String> {
        let directories = fs::read_dir(&self.repo_path).unwrap();
        let mut dirs_names: Vec<String> = Vec::new();
        for dir in directories {
            let tmp = dir.unwrap();
            // filter the entries to remove files and .git dir
            if tmp.path().is_dir() && tmp.file_name().ne(".git") {
                let dir_path = tmp.path().to_str().unwrap().to_string();
                logger::print_info(format!("Found directory: {}", dir_path));
                dirs_names.push(dir_path);
            }
        }
        dirs_names
    }

    pub fn get_repo_path(&self) -> &str {
        self.repo_path.as_str()
    }

    pub fn clone_repo(&mut self) {
        logger::print_job("Cloning down settings from ".to_string() + &self.upstream_url);
        match Repository::clone(&self.upstream_url, &self.repo_path) {
            Ok(repo) => {
                logger::print_info(format!("Cloned into {}", &self.repo_path));
                self.repo_path = repo.workdir().unwrap().to_str().unwrap().to_string()
            },
            Err(e) => panic!("Failed to clone: {}", e),
        }
    }
}
