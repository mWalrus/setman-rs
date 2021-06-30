// Copyright (c) 2021 Oskar Hellkvist <hellkvistoskar@protonmail.com>

// SPDX-License-Identifier: BSD-2-Clause

use crate::fileman;
use crate::gitman;
use crate::logger;
use crate::paths;
use crate::readline;

use fileman::{App, Apps};
use git2::Repository;
use gitman::GitRepo;
use paths::Paths;
use std::{fs::File, io::Read, path::{Path, PathBuf}};
use std::io::Error;

pub enum SetmanAction<'a> {
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

pub fn sync_settings(action: SetmanAction) {
    let settings_path = Paths::new().settings_path;
    let gitman = GitRepo::new();
    gitman.clone_repo(true);
    match action {
        SetmanAction::SyncUp => {
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
        }
        SetmanAction::SyncDown => {
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
            }
        }
        _ => println!("Invalid option, exiting."),
    }
}

pub fn print_app_list(option: ListOptions, verbose: bool) {
    logger::print_job("Applications:".to_string());

    let mut apps = Apps::new();
    match option {
        ListOptions::Literal(app_names) => {
            match app_names {
                Some(names) => {
                    for name in names {
                        let app = apps.find_app_by_name(name).unwrap();
                        logger::print_app(&app.name, &app.config_path, &app.file_names, verbose)
                    }
                },
                None => {
                    for app in apps.items {
                        logger::print_app(&app.name, &app.config_path, &app.file_names, verbose);
                    }
                }
            }
        },
        ListOptions::Regex(regex) => {
            let found_apps = apps.find_apps_from_regex(
                regex
            );
            for app in found_apps.unwrap() {
                logger::print_app(&app.name, &app.config_path, &app.file_names, verbose);
            }
        }
    }
}

fn copy_app_files(app: &App, from_local: bool) {
    let local_path = &Paths::new().get_app_path(&app.name);
    logger::print_job("Found application:".to_string());
    let tmp_app = app.clone();
    logger::print_app(&tmp_app.name, &tmp_app.config_path, &tmp_app.file_names, false);
    if from_local {
        fileman::copy_files(app.clone().file_names, local_path, &app.config_path).unwrap();
        return;
    }
    fileman::copy_files(app.clone().file_names, &app.config_path, local_path).unwrap();
}

pub fn app_action(action: SetmanAction) {
    let mut apps = Apps::new();
    match action {
        SetmanAction::Install(app_name) => {
            let app = apps.find_app_by_name(&app_name).unwrap();
            logger::print_job(format!("Installing {}", app_name));
            copy_app_files(&app, true);
        },
        SetmanAction::Uninstall(app_name) => {
            logger::print_job(format!("Uninstalling {}", app_name));
            let app = apps.find_app_by_name(app_name).unwrap();
            fileman::remove_files(&app.config_path).unwrap();
        },
        SetmanAction::Save(app_name) => {
            let app = apps.find_app_by_name(&app_name).unwrap();
            logger::print_job(format!("Saving {}", app_name));
            copy_app_files(&app, false);
        },
        SetmanAction::Modify(app_name) => {
            logger::print_job(format!("Modify {}", &app_name));
            modify_application(app_name).unwrap();
        },
        SetmanAction::Remove(app_name) => {
            readline::are_you_sure("remove ".to_string() + app_name).unwrap();
            logger::print_job(format!("Removing {}", &app_name));
            // remove app from saved list of apps
            apps.remove_app(app_name).unwrap();

            let app_local_path = Paths::new()
                .get_app_path(&app_name);
            // remove the application's files in the local copy of configs
            fileman::remove_files(&app_local_path).unwrap();
            logger::print_info("Done".to_string());
        },
        SetmanAction::New => {
            logger::print_new_app_header();
            let app_name = readline::read("Enter Application name").unwrap();
            let app_config_path = readline::read("Config path (relative to home)").unwrap();
            let config_files = readline::read("File names to save (space separated)").unwrap();

            let files_names = config_files.split_whitespace().map(String::from).collect();
            let mut apps = Apps::new();
            apps.save_new_app(App::new(app_name, app_config_path, files_names)).unwrap();
        },
        _ => panic!("Invalid option, exiting."),
    }
}

pub fn all_apps_action(action: SetmanAction) {
    let apps = Apps::new();

    for app in apps.items.iter() {
        match action {
            SetmanAction::InstallAll(apps_to_skip) => {
                logger::print_job("Installing all applications".to_owned());
                if !apps_to_skip.contains(&app.name) {
                    copy_app_files(app, true);
                }
            },
            SetmanAction::UninstallAll(apps_to_skip) => {
                logger::print_job("Uninstalling all applications".to_owned());
                if !apps_to_skip.contains(&app.name) {
                    fileman::remove_files(&app.config_path).unwrap();
                }
            },
            SetmanAction::SaveAll(apps_to_skip) => {
                logger::print_job("Saving all applications".to_owned());
                if !apps_to_skip.contains(&app.name) {
                    copy_app_files(app, false)
                }
            },
            _ => println!("Invalid option, exiting"),
        };
    }

}

pub fn modify_application(app_name: &str) -> Result<(), Error> {
    let mut apps = Apps::new();
    let mut app = apps.find_app_by_name(&app_name).unwrap();
    let mod_options = vec!["Name", "Config path", "File names"];
    match readline::select(mod_options.clone())? {
        0 => app.name = readline::read("Enter a new name")?,
        1 => {
            let rel_path = readline::read("Enter a new config path")?;
            let config_path = PathBuf::from(
                Paths::new().get_absolute_path(&rel_path)
            );
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
        _ => panic!("Invalid option, exiting."),
    }
    // make sure user wants to modify the application
    if readline::are_you_sure("modify ".to_owned() + &app_name)? {
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

    let local_commit_file = Paths::new().commit_id_path;
    let mut file = match File::open(&local_commit_file) {
        Ok(file) => file,
        Err(e) => panic!("Could not open {:?}: {}", &local_commit_file, e),
    };
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    if contents.eq(&commit_id.to_string()) {
        logger::print_info("Local is up to date".to_string());
        return
    }
    logger::print_warn("Local is behind".to_string());
}
