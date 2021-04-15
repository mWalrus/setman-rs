extern crate git2;
extern crate uuid;
extern crate toml;
extern crate serde;

#[path = "readline.rs"]
mod readline;
#[path = "logger.rs"]
mod logger;

use git2::{Repository, Signature, Time, Tree};
use uuid::Uuid;
use std::fs;
use std::process::exit;
use serde::Deserialize;
use std::time::{SystemTime, UNIX_EPOCH};

static GIT_FILE: &str = "git.toml";

#[derive(Deserialize, Clone)]
pub struct GitRepo {
    upstream_url: String,
    repo_path: String,
    author: Author,
}

#[derive(Deserialize, Clone)]
struct Author {
    name: String,
    email: String,
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
        let git_repo: GitRepo = match toml::from_str(&file_content) {
            Ok(git_repo) => git_repo,
            Err(_e) => {
                logger::print_warn("Upstream url not found, exiting".to_string());
                exit(0);
            }
        };
        let repo_name = "setman-tmp".to_string() + &Uuid::new_v4().to_string();
        GitRepo {
            upstream_url: git_repo.upstream_url,
            repo_path: "/tmp/".to_string() + &repo_name,
            author: git_repo.author,
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

    pub fn push_changes(self, commit_msg: &str) {
        match Repository::open(&self.repo_path) {
            Ok(repo) => {
                logger::print_info("Using existing repo: ".to_string() + &self.repo_path);
                let now = SystemTime::now();
                let since_epoch = now.duration_since(UNIX_EPOCH).expect("Time went backwards");
                let time_in_seconds = since_epoch.as_secs() as i64;
                logger::print_info(format!("Creating commit with message: {}", commit_msg));
                let author = Signature::new(&self.author.name, &self.author.email, &Time::new(time_in_seconds, 120));
                // TODO: stage a commit and execute it

                //let result = match repo.commit(Some("HEAD"), author, author, commit_msg, Tree::from(repo).id(), parents) {
                    //Ok(commit) => commit,
                    //Err(e) => println!("Error: {}", e),
                //};
            },
            Err(e) => panic!("Failed to open {} as a git repo: {}", &self.repo_path, e),
        }
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
