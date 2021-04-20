extern crate git2;
extern crate uuid;
extern crate toml;
extern crate serde;

#[path = "readline.rs"]
mod readline;
#[path = "logger.rs"]
mod logger;
#[path = "paths.rs"]
mod paths;

use git2::{Commit, Cred, Error, IndexAddOption, Oid, PushOptions, RemoteCallbacks, Repository, Signature, Tree};
use uuid::Uuid;
use std::fs;
use std::process::exit;
use serde::Deserialize;
use toml::Value;
use paths::Paths;

#[derive(Deserialize, Clone)]
pub struct GitRepo {
    upstream_url: String,
    repo_path: String,
}

// TODO: implement push functionality
impl GitRepo {
    pub fn new() -> GitRepo {
        let git_config_path = Paths::new().git_config_path;
        let file_content = match fs::read_to_string(&git_config_path) {
            Ok(content) => content,
            Err(_e) => {
                logger::print_warn(format!("File {} not found, exiting", git_config_path));
                exit(0);
            }
        };
        // maybe add test repo for dev purpose
        let upstream_url: String = match toml::from_str::<Value>(&file_content) {
            Ok(value) => value["upstream_url"].as_str().unwrap().to_string(),
            Err(_e) => {
                logger::print_warn("Upstream url not found, exiting".to_string());
                exit(0);
            }
        };
        let repo_name = "setman-tmp-".to_string() + &Uuid::new_v4().to_string();
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

    pub fn push_changes(self) -> Result<(), Error>{
        match Repository::open(&self.repo_path) {
            Ok(repo) => {
                logger::print_info("Using existing repo: ".to_string() + &self.repo_path);

                let signature = repo.signature()?;
                let mut index = repo.index().expect("Failed to get repo index");
                // git add .
                logger::print_job("Staging files for commit".to_string());
                index.add_all(["."].iter(), IndexAddOption::DEFAULT, None)?;
                index.write()?;

                // get index tree
                let tree_id = index.write_tree()?;
                let tree = repo.find_tree(tree_id)?;

                let parent = self.get_parent_commit(&repo);
                self.create_commit(&repo, &signature, &tree, &parent).unwrap();

                let mut push_opts = self.gen_push_opts(&signature);

                // push to remote origin
                let mut origin = repo.find_remote("origin")?;
                logger::print_job(format!("Pushing to remote: {}", origin.name().unwrap()));
                origin.push(&["refs/heads/main"], Some(&mut push_opts))?;
                logger::print_info("Done!".to_string());
                Ok(())
            },
            Err(e) => panic!("Failed to open {} as a git repo: {}", &self.repo_path, e),
        }
    }

    fn create_commit(&self, repo: &Repository, signature: &Signature, tree: &Tree, parent: &Commit) -> Result<(), Error> {
        let commit_msg = readline::read("Enter a commit message");
        let pretty_message = git2::message_prettify(commit_msg, None)?;
        let new_commit_id: Oid = match repo.commit(
            Some("HEAD"),
            &signature,
            &signature,
            &pretty_message,
            &tree, &[parent])
        {
            Ok(commit) => commit,
            Err(e) => {
                println!("Failed to create commit: {}", e);
                exit(0);
            },
        };
        logger::print_info(format!("Created new commit with id: {}", new_commit_id));
        Ok(())
    }

    fn gen_push_opts<'a>(&self, signature: &'a Signature) -> PushOptions<'a> {
        let mut callbacks = RemoteCallbacks::new();
        callbacks.credentials(move |_str, _option, _cred_type| {
            let password = readline::password("Enter your git password");
            Cred::userpass_plaintext(signature.name().unwrap(), &password)
        });

        let mut push_opts = PushOptions::new();
        push_opts.remote_callbacks(callbacks);
        push_opts
    }

    fn get_parent_commit<'a>(&self, repo: &'a Repository) -> Commit<'a> {
        match repo.revparse_single("origin") {
            Ok(obj) => obj,
            Err(e) => {
                eprintln!("Error: {}", e);
                exit(0)
            },
        }.as_commit().unwrap().to_owned()
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
