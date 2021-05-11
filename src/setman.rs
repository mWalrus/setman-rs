// Copyright (c) 2021 Oskar Hellkvist <hellkvistoskar@protonmail.com>

// SPDX-License-Identifier: BSD-2-Clause

use crate::fileman;
use crate::gitman;
use crate::logger;
use crate::paths;
use crate::readline;

use fileman::{App, Apps};
use gitman::GitRepo;
use paths::Paths;
use std::path::Path;
use std::{io::Error, process::exit};

pub fn check_path_existance() {
    let paths = Paths::new();
    fileman::path_exists(&paths.user_conf_path);
    fileman::path_exists(&paths.settings_path);
}

pub fn sync_settings(direction: &str) {
    let settings_path = &Paths::new().settings_path;
    let mut gitman = GitRepo::new();
    gitman.clone_repo();
    let repo_path = gitman.get_repo_path();
    match direction.eq("up") {
        true => {
            let dir_names = fileman::get_dir_names_in_path(&settings_path).unwrap();
            let mut apps = Apps::new();
            for dir_name in dir_names {
                let app = apps.find_app_by_name(&dir_name).unwrap();
                let source = format!("{}/{}", &settings_path, &app.name);
                let dest = format!("{}/{}", &repo_path, &app.name);
                fileman::copy_files(app.file_names, &source, &dest).unwrap();
            }
            gitman.push_changes().unwrap();
        }
        false => {
            let dirs_to_copy = gitman.clone().get_dir_names();
            for dir in dirs_to_copy.clone() {
                let source = format!("{}/{}", &repo_path, &dir);
                let dest = format!("{}/{}", &settings_path, &dir);
                let files = Path::new(&source).read_dir().unwrap();
                let file_names = files
                    .into_iter()
                    .map(|n| n.unwrap().file_name().to_str().unwrap().to_string())
                    .collect();
                fileman::copy_files(file_names, &source, &dest).unwrap();
            }
        }
    }
}

pub fn take_new_application() {
    logger::print_new_app_header();
    let app_name = readline::read("Enter Application name").unwrap();
    let app_config_path = readline::read("Config path (relative to home)").unwrap();
    let config_files = readline::read("File names to save (space separated)").unwrap();

    let files_names = config_files.split_whitespace().map(String::from).collect();
    let mut apps = Apps::new();
    apps.save_new_app(App::new(app_name, app_config_path, files_names));
}

pub fn print_app_list(app_names: Option<Vec<&str>>, verbose: bool) {
    logger::print_info("Applications:".to_string());
    let mut apps = Apps::new();
    if app_names != None {
        for name in app_names.unwrap() {
            let app = apps.find_app_by_name(&name).unwrap();
            logger::print_app(app.name, app.config_path, app.file_names, verbose);
        }
        return;
    }
    for app in apps.items {
        logger::print_app(app.name, app.config_path, app.file_names, verbose);
    }
}

fn copy_app_files(app: &App, from_local: bool) {
    let paths = Paths::new();
    let app_local_path = paths.clone().get_app_path(&app.name);
    let app_conf_path = paths.clone().get_absolute_path(&app.config_path);
    logger::print_job("Found application:".to_string());
    let tmp_app = app.clone();
    logger::print_app(tmp_app.name, tmp_app.config_path, tmp_app.file_names, false);
    if from_local {
        fileman::copy_files(app.clone().file_names, &app_local_path, &app_conf_path).unwrap();
        return;
    }
    fileman::copy_files(app.clone().file_names, &app_conf_path, &app_local_path).unwrap();
}

fn app_action(message: String, app_name: &str, from_local: bool) {
    logger::print_job(message);
    let mut apps = Apps::new();
    let app = apps.find_app_by_name(app_name).unwrap();
    copy_app_files(&app, from_local);
}

fn all_apps_action(message: String, apps_to_skip: Vec<&str>, from_local: bool) {
    logger::print_job(message);
    let apps = Apps::new();
    for app in apps.items.iter() {
        if !apps_to_skip.contains(&app.name.as_str()) {
            copy_app_files(app, from_local);
        }
    }
}

pub fn save_application(app_name: &str) {
    app_action(
        format!("Saving application {} to local collection", app_name),
        app_name,
        false,
    );
}

pub fn install_application(app_name: &str) {
    app_action(
        format!("Installing application {}", app_name),
        app_name,
        true,
    );
}

pub fn save_all_applications(apps_to_skip: Vec<&str>) {
    all_apps_action(
        "Saving all applications' settings".to_owned(),
        apps_to_skip,
        false,
    );
}

pub fn install_all_applications(apps_to_skip: Vec<&str>) {
    all_apps_action(
        "Installing all applications' settings".to_owned(),
        apps_to_skip,
        true,
    );
}

fn uninstall_pre(ru_sure_action: String, job_msg: String) -> Apps {
    if !readline::are_you_sure(ru_sure_action).unwrap() {
        logger::print_info("Exiting".to_owned());
        exit(0);
    }
    logger::print_job(job_msg);
    Apps::new()
}

pub fn uninstall_application(app_name: &str) {
    let mut apps = uninstall_pre(
        "uninstall ".to_owned() + &app_name,
        "Uninstalling ".to_owned() + &app_name,
    );
    let app = apps.find_app_by_name(&app_name).unwrap();
    fileman::remove_files(&Paths::new().get_app_path(&app.name)).unwrap();
}

pub fn uninstall_all_applications(apps_to_skip: Vec<&str>) {
    let apps = uninstall_pre(
        "uninstall all applications' settings".to_owned(),
        "Uninstalling all applications".to_owned(),
    );
    let paths = Paths::new();
    for app in apps.items.iter() {
        if !apps_to_skip.contains(&app.name.as_str()) {
            let conf_path = paths.clone().get_app_path(&app.name);
            let tmp_app = app.clone();
            logger::print_app(tmp_app.name, tmp_app.config_path, tmp_app.file_names, true);
            fileman::remove_files(&conf_path).unwrap();
        }
    }
}

pub fn remove_application(app_name: &str) {
    logger::print_warn("Removing ".to_owned() + &app_name);
    readline::are_you_sure("remove ".to_string() + app_name).unwrap();
    let mut apps = Apps::new();
    apps.remove_app(app_name);
    logger::print_info("Done".to_string());
}

pub fn modify_application(app_name: &str) -> Result<(), Error> {
    let mut apps = Apps::new();
    let mut app = apps.find_app_by_name(&app_name).unwrap();
    logger::print_job("Modify ".to_owned() + &app_name);
    let mod_options = vec!["Name", "Config path", "File names"];
    match readline::select(mod_options.clone())? {
        0 => app.name = readline::read("Enter a new name")?,
        1 => app.config_path = readline::read("Enter a new config path")?,
        2 => {
            let mut file_names = app.file_names;
            let file_names_str = file_names.iter().map(|n| n.as_str()).collect();
            let file_index: usize = readline::select(file_names_str)?;

            let new_file_name = readline::read("Enter a new file name")?;
            file_names.remove(file_index);
            file_names.insert(file_index, new_file_name);
            app.file_names = file_names;
        }
        _ => {
            logger::print_warn("Invalid option, exiting.".to_owned());
            exit(0);
        }
    }
    // make sure user wants to modify the application
    if readline::are_you_sure("modify ".to_owned() + &app_name)? {
        apps.remove_app(app_name);
        apps.save_new_app(app);
    };
    Ok(())
}
