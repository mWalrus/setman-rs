// Copyright (c) 2021 Oskar Hellkvist <hellkvistoskar@protonmail.com>

// SPDX-License-Identifier: BSD-2-Clause

use crate::fileman;
use crate::gitman;
use crate::paths;
use crate::readline;
use crate::thiserror;

use fileman::{App, Apps};
use git2::Repository;
use gitman::GitRepo;
use paths::Paths;
use std::{fs::File, io::Read, path::Path};
use std::io::Error as IOError;
use thiserror::Error;

pub enum SetManAction<'a> {
    Install(&'a str),
    Uninstall(&'a str),
    Save(&'a str),
    Modify(&'a str),
    Remove(&'a str),
    InstallAll(&'a Vec<String>),
    UninstallAll(&'a Vec<String>),
    SaveAll(&'a Vec<String>),
    New,
    SyncUp,
    SyncDown,
}

pub enum ListOptions<'a> {
    Literal(&'a Option<Vec<&'a str>>),
    Regex(&'a str),
}

#[derive(Error, Debug)]
pub enum SetManError {
    #[error("Invalid option")]
    InvalidOption,
}

pub fn sync_settings(action: SetManAction) -> Result<(), SetManError> {
    let settings_path = Paths::default().settings_path;
    let gitman = GitRepo::new();
    gitman.clone_repo(true);
    match action {
        SetManAction::SyncUp => {
            let dir_names = fileman::get_dir_names_in_path(&settings_path).unwrap();
            let mut apps = Apps::new();
            for dir_name in dir_names {
                let mut source = settings_path.clone();
                source.push(&dir_name);
                let mut dest = gitman.repo_path.clone();
                dest.push(&dir_name);
                let app = apps.find_app_by_name(&dir_name).unwrap();
                fileman::copy_files(app.file_names, &source, &dest).unwrap();
            }
            gitman.push_changes().unwrap();
            Ok(())
        },
        SetManAction::SyncDown => {
            let dirs_to_copy = gitman.get_dir_names();
            for dir_name in dirs_to_copy.clone() {
                let mut source = gitman.repo_path.clone();
                source.push(&dir_name);
                let mut dest = settings_path.clone();
                dest.push(&dir_name);
                let files = Path::new(&source).read_dir().unwrap();
                let file_names = files
                    .into_iter()
                    .map(|n| n.unwrap().file_name().to_str().unwrap().to_string())
                    .collect();
                fileman::copy_files(file_names, &source, &dest).unwrap();
            };
            Ok(())
        },
        _ => Err(SetManError::InvalidOption),
    }
}

pub fn print_app_list(option: ListOptions, verbose: bool) {
    job!("Applications:");

    let mut apps = Apps::new();
    match option {
        ListOptions::Literal(app_names) => {
            match app_names {
                Some(names) => {
                    for name in names {
                        let app = apps.find_app_by_name(name).unwrap();
                        print_app!(app, verbose)
                    }
                },
                None => {
                    for app in apps.items {
                        print_app!(app, verbose);
                    }
                }
            }
        },
        ListOptions::Regex(regex) => {
            let found_apps = apps.find_apps_from_regex(
                regex
            );
            for app in found_apps.unwrap() {
                print_app!(app, verbose);
            }
        }
    }
}

fn copy_app_files(app: &App, from_local: bool) {
    let mut local_path = Paths::default().settings_path;
    local_path.push(&app.name);
    job!("Copying files for {}", &app.name);
    if from_local {
        fileman::copy_files(app.clone().file_names, &local_path, &app.config_path).unwrap();
        return;
    }
    fileman::copy_files(app.clone().file_names, &app.config_path, &local_path).unwrap();
}

pub fn app_action(action: SetManAction) {
    let mut apps = Apps::new();
    match action {
        SetManAction::Install(app_name) => {
            let app = apps.find_app_by_name(&app_name).unwrap();
            job!("Installing {}", app_name);
            copy_app_files(&app, true);
        },
        SetManAction::Uninstall(app_name) => {
            job!("Uninstalling {}", app_name);
            let app = apps.find_app_by_name(app_name).unwrap();
            fileman::remove_files(&app.config_path).unwrap();
        },
        SetManAction::Save(app_name) => {
            let app = apps.find_app_by_name(&app_name).unwrap();
            job!("Saving {}", app_name);
            copy_app_files(&app, false);
        },
        SetManAction::Modify(app_name) => {
            job!("Modify {}", &app_name);
            modify_application(app_name).unwrap();
        },
        SetManAction::Remove(app_name) => {
            readline::are_you_sure("remove ".to_string() + app_name).unwrap();
            job!("Removing {}", &app_name);
            // remove app from saved list of apps
            apps.remove_app(app_name).unwrap();

            let mut app_local_path = Paths::default().settings_path;
            app_local_path.push(&app_name);
            // remove the application's files in the local copy of configs
            fileman::remove_files(&app_local_path).unwrap();
            info!("Done");
        },
        SetManAction::New => {
            info!("New application:");
            let app_name = readline::read("Enter Application name").unwrap();
            info!("Config path should be relative to home");
            let app_config_path = readline::read("Config path").unwrap();
            info!("Format: file_name.extension (space separated if > 1)");
            let config_files = readline::read("File name(s) to save").unwrap();

            let files_names = config_files.split_whitespace().map(String::from).collect();
            apps.save_new_app(App::new(app_name, app_config_path, files_names)).unwrap();
        },
        _ => panic!("{}", SetManError::InvalidOption),
    }
}

pub fn all_apps_action(action: SetManAction) {
    let apps = Apps::new();

    for app in apps.items.iter() {
        match action {
            SetManAction::InstallAll(apps_to_skip) => {
                if !apps_to_skip.contains(&app.name) {
                    copy_app_files(app, true);
                }
            },
            SetManAction::UninstallAll(apps_to_skip) => {
                if !apps_to_skip.contains(&app.name) {
                    fileman::remove_files(&app.config_path).unwrap();
                }
            },
            SetManAction::SaveAll(apps_to_skip) => {
                if !apps_to_skip.contains(&app.name) {
                    copy_app_files(app, false)
                }
            },
            _ => panic!("{}", SetManError::InvalidOption),
        };
    }

}

pub fn modify_application(app_name: &str) -> Result<(), IOError> {
    let mut apps = Apps::new();
    let mut app = apps.find_app_by_name(&app_name).unwrap();
    let mod_options = vec!["Name", "Config path", "File names"];
    match readline::select(mod_options.clone())? {
        0 => app.name = readline::read("Enter a new name")?,
        1 => {
            let rel_path = readline::read("Enter a new config path")?;
            let config_path = paths::get_absolute_path(&rel_path);
            app.config_path = config_path;
        },
        2 => {
            let mut file_names = app.file_names;
            let file_names_str = file_names
                .iter()
                .map(|s| &**s)
                .collect();
            let file_index: usize = readline::select(file_names_str)?;

            let new_file_name = readline::read("Enter a new file name")?;
            file_names.remove(file_index);
            file_names.insert(file_index, new_file_name);
            app.file_names = file_names;
        }
        _ => panic!("{}", SetManError::InvalidOption),
    }
    // make sure user wants to modify the application
    if readline::are_you_sure("modify ".to_owned() + app_name)? {
        apps.remove_app(app_name)?;
        apps.save_new_app(app)?;
    };
    Ok(())
}

pub fn compare_upstream() {
    // get latest commit from upstream and get its id
    let git_repo = gitman::GitRepo::new();
    git_repo.clone_repo(false);
    let repo = Repository::open(&git_repo.repo_path).unwrap();
    let commit_id = git_repo
        .get_parent_commit(&repo)
        .unwrap()
        .id()
        .to_string();

    let local_commit_file = Paths::default().commit_id_path;
    let mut file = match File::open(&local_commit_file) {
        Ok(file) => file,
        Err(e) => panic!("Could not open {:?}: {}", &local_commit_file, e),
    };
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    if contents.eq(&commit_id) {
        info!("Local is up to date");
        return
    }
    warn!("Local is behind");
}
