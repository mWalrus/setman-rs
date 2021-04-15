extern crate git2;
extern crate uuid;
extern crate toml;
extern crate serde;
extern crate home;

#[path = "readline.rs"]
mod readline;
#[path = "logger.rs"]
mod logger;

use git2::{Error, IndexAddOption, Oid, PushOptions, Repository, RepositoryState};
use uuid::Uuid;
use std::fs;
use std::process::exit;
use serde::Deserialize;
use toml::Value;
use home::home_dir;

static GIT_FILE: &str = ".config/setman/git.toml";

#[derive(Deserialize, Clone)]
pub struct GitRepo {
    upstream_url: String,
    repo_path: String,
}

// TODO: implement push functionality
impl GitRepo {
    pub fn new() -> GitRepo {
        let git_file_absolute = home_dir().unwrap().display().to_string() + GIT_FILE;
        let file_content = match fs::read_to_string(&git_file_absolute) {
            Ok(content) => content,
            Err(_e) => {
                logger::print_warn(format!("File {} not found, exiting", git_file_absolute));
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
        GitRepo {
            upstream_url,
            repo_path: "/tmp/".to_string() + &repo_name,
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

    pub fn push_changes(self, commit_msg: &str) -> Result<(), Error>{
        match Repository::open(&self.repo_path) {
            Ok(repo) => {
                logger::print_info("Using existing repo: ".to_string() + &self.repo_path);
                let signature = repo.signature()?;
                let pretty_message = git2::message_prettify(commit_msg, None)?;
                let mut index = repo.index().expect("Failed to get repo index");
                // Simulate git add *
                logger::print_info("Staging files for commit".to_string());
                index.add_all(["*"].iter(), IndexAddOption::DEFAULT, None)?;
                index.write()?;

                // if no changes have been made we skip pushing to upstream
                if repo.state() == RepositoryState::Clean {
                    logger::print_info(format!("Worktree for repo {} is clean, skipping push", &self.repo_path));
                    exit(0);
                }

                // get previous commit
                let obj = repo.revparse_single("main")?;
                let prev_commit = obj.as_commit().unwrap();
                let tree = prev_commit.tree().unwrap();

                // Create commit
                let new_commit_id: Oid = match repo.commit(
                    Some("HEAD"),
                    &signature,
                    &signature,
                    &pretty_message,
                    &tree, &[prev_commit])
                {
                    Ok(commit) => commit,
                    Err(_e) => {
                        println!("Failed to create commit");
                        exit(0);
                    },
                };
                logger::print_info(format!("Created new commit with id: {}", new_commit_id));

                // push to remote origin
                let mut origin = repo.find_remote("origin")?;
                origin.push(&["main"], Some(&mut PushOptions::new()))?;
                Ok(())
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
