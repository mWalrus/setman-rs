// this file will be used to clone and push settings to a specified upsteam url using git

// When a user is using the app, they should be given the option to fetch their configs from their upstream url

// This url can be set using the --set-upstream option, if upstream isnt set when the user is using the app
// the application will complain.

// When the upstream is set the user can chose to sync their settings, either clone from upstream git url or
// push their local changes to git

// git management will be separate from the local settings directory
// The local folder will be in setman's .config directory.
// When we are doing git clones or pushes we create a temporary git repo in /tmp
// this is done to remove the need for a permanent git folder in the .config directory

// I might think about redoing the sync command. So instead of a general sync command i could
// do two separate commands. One for push syncs and one for clone syncs. They should probably still
// be application specific with the option to sync all

// i will need to think more about the above thoughts

// The clone sync flag could have application specific syncing so when the repo is cloned
// setman will check if an application was specified and only copy the cloned files for that application
// to the local collection

// Same thing could be done for the push command, an option to sync all will be avaliable but application specific
// push syncs is also possible, application settings will be copied to local and pushed.

// The option to only sync a single application both ways is good if you have unfinished versions of a config you
// are not sure if you want to use or discard on another system or user.

// expand this to be able to use ssh for clone and push
// (repository struct does not support this but RepoBuilder does)

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
use std::{borrow::Borrow, fs};
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
        logger::print_job("Cloning down settings from ".to_string() + self.upstream_url.borrow());
        match Repository::clone(self.upstream_url.borrow(), &(self.repo_path)) {
            Ok(repo) => {
                logger::print_info(format!("Cloned into {}", &(self.repo_path)));
                self.repo_path = repo.workdir().unwrap().to_str().unwrap().to_string()
            },
            Err(e) => panic!("Failed to clone: {}", e),
        }
    }
}
