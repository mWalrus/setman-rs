// Copyright (c) 2021 Oskar Hellkvist <hellkvistoskar@protonmail.com>

// SPDX-License-Identifier: BSD-2-Clause

use crate::logger;
use crate::paths;
use crate::readline;

use git2::{
    build::RepoBuilder, Commit, Cred, Error, FetchOptions, IndexAddOption, Oid, PushOptions,
    RemoteCallbacks, Repository, Signature, Tree,
};
use paths::Paths;
use serde::Deserialize;
use std::{fs::File, io::{LineWriter, Write}, process::exit};
use std::{fs, path::Path};
use uuid::Uuid;
use std::path::PathBuf;

#[derive(Deserialize, Clone)]
pub struct GitRepo {
    repo_path: PathBuf,
    git_settings: GitSettings,
}

#[derive(Deserialize, Clone, Debug)]
struct GitSettings {
    upstream_url: String,
    name: String,
    email: String,
    pass: Option<String>,
}

impl GitSettings {
    fn new() -> GitSettings {
        let git_config_path = Paths::new().git_config_path;
        let file_content = match fs::read_to_string(&git_config_path) {
            Ok(content) => content,
            Err(_e) => {
                panic!("File {} not found, exiting", git_config_path.to_str().unwrap());
            }
        };
        match toml::from_str::<Self>(&file_content) {
            Ok(settings) => Self {
                upstream_url: settings.upstream_url,
                name: settings.name,
                email: settings.email,
                pass: settings.pass,
            },
            Err(e) => {
                panic!("Could not parse {}. Error: {}", git_config_path.to_str().unwrap(), e);
            }
        }
    }
}

impl GitRepo {
    pub fn new() -> Self {
        let git_settings = GitSettings::new();
        let tmp_dir_name = "setman-tmp-".to_string() + &Uuid::new_v4().to_string();
        let repo_path: PathBuf = [
            r"/tmp",
            &tmp_dir_name,
        ].iter().collect();
        Self {
            repo_path,
            git_settings,
        }
    }

    pub fn get_dir_names(self) -> Vec<String> {
        logger::print_job("Getting directories from git repo".to_string());
        let directories = fs::read_dir(&self.repo_path).unwrap();

        let mut dirs_names: Vec<String> = Vec::new();

        for dir in directories {
            let tmp = dir.unwrap();
            // filter the entries to remove files and .git dir
            if tmp.path().is_dir() && tmp.file_name().ne(".git") {
                let dir_path = tmp.file_name().to_str().unwrap().to_string();
                logger::print_info(format!("Found directory: {}", dir_path));
                dirs_names.push(dir_path);
            }
        }
        dirs_names
    }

    pub fn get_repo_path(&self) -> &str {
        self.repo_path.to_str().unwrap()
    }

    pub fn push_changes(self) -> Result<(), Error> {
        match Repository::open(&self.repo_path) {
            Ok(repo) => {
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
                let new_commit_id = self.create_commit(&repo, &signature, &tree, &parent)
                    .unwrap();

                self.save_commit_id(new_commit_id).unwrap();

                let callbacks = self.gen_callbacks();
                let mut push_opts = PushOptions::new();
                push_opts.remote_callbacks(callbacks);

                // push to remote origin
                let mut origin = repo.find_remote("origin")?;
                logger::print_job(format!("Pushing to remote: {}", origin.name().unwrap()));
                origin.push(&["refs/heads/main"], Some(&mut push_opts))?;
                logger::print_info("Done!".to_string());
                Ok(())
            }
            Err(e) => panic!("Failed to open {} as a git repo: {}", &self.get_repo_path(), e),
        }
    }

    fn create_commit(
        &self,
        repo: &Repository,
        signature: &Signature,
        tree: &Tree,
        parent: &Commit,
    ) -> Result<Oid, Error> {
        let commit_msg = readline::read("Enter a commit message").unwrap();
        let pretty_message = git2::message_prettify(commit_msg, None)?;
        let new_commit_id: Oid = match repo.commit(
            Some("HEAD"),
            &signature,
            &signature,
            &pretty_message,
            &tree,
            &[parent],
        ) {
            Ok(commit) => commit,
            Err(e) => {
                println!("Failed to create commit: {}", e);
                exit(0);
            }
        };
        logger::print_info(format!("Created new commit with id: {}", new_commit_id));
        Ok(new_commit_id)
    }

    fn save_commit_id(&self, commit_id: Oid) -> std::io::Result<()> {
        logger::print_job("Saving new commit id".to_string());
        let commit_id_path = Paths::new().commit_id_path;
        let file = File::create(commit_id_path)?;
        let mut file = LineWriter::new(file);
        file.write_all(commit_id.to_string().as_bytes())?;
        file.flush()?;
        Ok(())
    }

    fn gen_callbacks<'a>(&'a self) -> RemoteCallbacks<'a> {
        let mut callbacks = RemoteCallbacks::new();
        callbacks.credentials(move |_str, _option, _cred_type| {
            //let password = readline::password("Enter your git password").unwrap();
            let password: &str = &self
                .clone()
                .git_settings
                .pass
                .unwrap_or_else(|| readline::password("Enter your git password").unwrap());
            Cred::userpass_plaintext(&self.git_settings.name, &password)
        });
        callbacks
    }

    pub fn get_parent_commit<'a>(self, repo: &'a Repository) -> Commit<'a> {
        let commit: Commit = match repo.revparse_single("origin") {
            Ok(obj) => obj,
            Err(e) => {
                eprintln!("Error: {}", e);
                exit(0)
            }
        }
        .as_commit()
        .unwrap()
        .to_owned();
        commit
    }

    pub fn clone_repo(&mut self) {
        logger::print_job("Cloning down from upstream".to_owned());

        let callbacks = self.gen_callbacks();
        let mut fetch_opts = FetchOptions::new();
        fetch_opts.remote_callbacks(callbacks);

        let mut builder = RepoBuilder::new();
        builder.fetch_options(fetch_opts);

        let repo = builder
            .clone(&self.git_settings.upstream_url, Path::new(&self.repo_path))
            .unwrap();

        let latest_commit = self.get_parent_commit(&repo);
        self.save_commit_id(latest_commit.id());
    }
}
