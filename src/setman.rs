extern crate colored;
extern crate toml;
extern crate serde;
extern crate home;

#[path = "logger.rs"]
mod logger;
#[path = "fileman.rs"]
mod fileman;
#[path = "readline.rs"]
mod readline;
#[path = "gitman.rs"]
mod gitman;
#[path = "paths.rs"]
mod paths;

use std::process::exit;
use fileman::{Apps, App};
use gitman::GitRepo;
use colored::*;
use paths::Paths;

pub fn check_path_existance() {
    let paths = Paths::new();
    fileman::dir_exists(&paths.user_conf_path);
    fileman::dir_exists(&paths.settings_path);
}

pub fn sync_settings(direction: &str) {
    let settings_path = &Paths::new().settings_path;
    let mut gitman = GitRepo::new();
    gitman.clone_repo();
    let repo_path = gitman.get_repo_path();
    match direction.eq("up") {
        true => {
            let dir_names = fileman::get_dir_names_in_path(&settings_path);
            let mut apps = Apps::new();
            for dir_name in dir_names {
                let app = apps.find_app_by_name(&dir_name).unwrap();
                let file_names = app.file_names;
                let source = settings_path.to_string() + "/" + &app.name;
                let dest = repo_path.to_string() + &app.name;
                fileman::copy_files(file_names, &source, &dest).unwrap();
            }
            gitman.push_changes().unwrap();
        },
        false => {
            let dirs_to_copy = gitman.clone().get_dir_names();
            fileman::copy_files(
                dirs_to_copy.to_owned(),
                repo_path,
                &settings_path
            ).unwrap();
        }
    }
}

pub fn take_new_application() {
    logger::print_new_app_header();
    let app_name = readline::read("Enter Application name");
    let app_config_path = readline::read("Config path (relative to home)");
    let config_files = readline::read("File names to save (space separated)");

    let files_names = config_files.split_whitespace()
        .map(|f| f.to_string()).collect();
    let mut apps = Apps::new();
    apps.save_new_app(App::new(app_name, app_config_path, files_names));
}

fn print_app(app: App, verbose: bool) {
    println!("  {} {}", "-".bold().bright_purple(), app.name.bold());
    if verbose {
        println!("      {} {}", "Config path =>".bold().bright_cyan(), app.config_path);
        println!("      {}", "File names:".bold().bright_green());
        for file in app.file_names {
            println!("          {} {}", "=>".bold().bright_green(), file);
        }
    }
}

pub fn print_app_list(app_names: Option<Vec<&str>>, verbose: bool) {
    logger::print_info("Applications:".to_string());
    let mut apps = Apps::new();
    if app_names != None {
        for name in app_names.unwrap() {
            let app = apps.find_app_by_name(&name).unwrap();
            print_app(app, verbose);
        }
        return
    }
    for app in apps.items {
        print_app(app, verbose);
    }
}

fn app_copy_action(app: &App, from_local: bool) {
    let paths = Paths::new();
    let app_local_path = paths.clone().get_app_path(&app.name);
    let app_conf_path = paths.clone().get_absolute_path(&app.config_path);
    logger::print_job("Found application:".to_string());
    print_app(app.to_owned(), true);
    if from_local {
        fileman::copy_files(app.clone().file_names, &app_local_path, &app_conf_path).unwrap();
        return
    }
    fileman::copy_files(app.clone().file_names, &app_conf_path, &app_local_path).unwrap();
}

pub fn save_application(app_name: &str) {
    logger::print_job("Saving application ".to_owned() + &app_name + " to local collection");
    let mut apps = Apps::new();
    let app = apps.find_app_by_name(app_name).unwrap();
    app_copy_action(&app, false);
}

pub fn save_all_applications(apps_to_skip: Vec<&str>) {
    logger::print_job("Saving all applications' settings".to_owned());
    let apps = Apps::new();
    for app in apps.items.iter() {
        if !apps_to_skip.contains(&app.name.as_str()) {
            app_copy_action(app, false);
        }
    }
}

pub fn install_application(app_name: &str) {
    logger::print_job("Installing application ".to_owned() + &app_name);
    let mut apps = Apps::new();
    let app = apps.find_app_by_name(app_name).unwrap();
    app_copy_action(&app, true);
}

pub fn install_all_applications(apps_to_skip: Vec<&str>) {
    logger::print_job("Installing all applications' settings".to_owned());
    let apps = Apps::new();
    for app in apps.items.iter() {
        if !apps_to_skip.contains(&app.name.as_str()) {
            app_copy_action(app, true);
        }
    }
}

fn uninstall_pre(ru_sure: String, job_msg: String) -> Apps {
    let ans = readline::are_you_sure(ru_sure);
    if !ans {
        logger::print_info("Exiting".to_owned());
        exit(0);
    }
    logger::print_job(job_msg);
    Apps::new()
}

pub fn uninstall_application(app_name: &str) {
    let mut apps = uninstall_pre(
        "uninstall ".to_owned() + &app_name,
        "Uninstalling ".to_owned() + &app_name);
    let app = apps.find_app_by_name(&app_name).unwrap();
    fileman::remove_files(&Paths::new().get_app_path(&app.name));
}

pub fn uninstall_all_applications(apps_to_skip: Vec<&str>) {
    let apps = uninstall_pre(
        "uninstall all applications' settings".to_owned(),
        "Uninstalling all applications".to_owned());
    let paths = Paths::new();
    for app in apps.items.iter() {
        if !apps_to_skip.contains(&app.name.as_str()) {
            let conf_path = paths.clone().get_app_path(&app.name);
            print_app(app.to_owned(), true);
            fileman::remove_files(&conf_path);
        }
    }
}

pub fn remove_application(app_name: &str) {
    logger::print_warn("Removing ".to_owned() + &app_name);
    readline::are_you_sure("Remove ".to_string() + app_name);
    let mut apps = Apps::new();
    apps.remove_app(app_name);
}

pub fn modify_application(app_name: &str) {
    let mut apps = Apps::new();
    let mut app = apps.find_app_by_name(&app_name).unwrap();
    logger::print_job("Modify ".to_owned() + &app_name);
    let mod_options = vec!["Name", "Config path", "File names"];
    match readline::select(mod_options.clone()) {
        0 => app.name = readline::read("Enter a new name"),
        1 => app.config_path = readline::read("Enter a new config path"),
        2 => {
            let mut file_names = app.file_names;
            let file_names_str = file_names.iter().map(|name| name.as_str()).collect();
            let file_index: usize = readline::select(file_names_str);

            let new_file_name = readline::read("Enter a new file name");
            file_names.remove(file_index);
            file_names.insert(file_index, new_file_name);
            app.file_names = file_names;

        },
        _ => {
            logger::print_warn("Invalid option, exiting.".to_owned());
            exit(0);
        },
    }
    // make sure user wants to modify the application
    if readline::are_you_sure("modify ".to_owned() + &app_name) {
        apps.remove_app(app_name);
        apps.save_new_app(app);
    }
}
