extern crate colored;
extern crate toml;
extern crate serde;
extern crate home;


#[path = "./logger.rs"]
mod logger;
#[path = "./fileman.rs"]
mod fileman;
#[path = "./readline.rs"]
mod readline;

use fileman::{Apps, App};
use colored::*;

pub fn take_new_application() {
    println!("{} {}", "[-]".blue().bold(), "New application:".bold());
    let app_name = readline::read("Enter Application name: ");
    let app_config_path = readline::read("Config path (relative to home): ");
    let config_files = readline::read("File names to save (space separated): ");

    let files_names = config_files.split_whitespace()
        .map(|f| f.to_string()).collect();
    let mut apps = Apps::new();
    apps.save_new_app((app_name, app_config_path, files_names));
}

// Gets the absolute path to a config directory
fn get_absolute_path(relative_path: &str) -> String {
    home::home_dir().unwrap().display().to_string() + "/" + relative_path + "/"
}

pub fn print_app_list() {
    logger::print_info("Found applications:".to_owned());
    let apps = Apps::new();
    for app in apps.items.iter() {
        println!("    - {}", app.name);
    }
}

fn are_you_sure(action: String, yes_favored: bool) -> bool {
    let formatted = if yes_favored {"Y/n"} else {"y/N"};
    let ans = readline::read(&format!("Are you sure you want to {}? ({}): ", &action, formatted));
    match ans.to_lowercase().as_str() {
        "y" | "yes" => return true,
        "n" | "no" => return false,
        "" => return yes_favored,
        _ => return false
    }
}

// Prints general information about an app
fn print_app_info(conf_path: &str, file_names: Vec<String>) {
    logger::print_info("Found app config path: ".to_owned() + conf_path);
    logger::print_info("Found files to handle:".to_owned());
    for name in file_names.iter() {
        println!("    {} {}", "=>".bold(), name);
    }
}

fn get_paths(app: &App) -> (String, String) {
    return (get_absolute_path(&app.config_path), "./".to_string() + &app.name + "/")
}

fn app_copy_action(app: &App, from_local: bool) {
    let (conf_path, rel_path) = get_paths(app);
    print_app_info(&conf_path, app.clone().file_names);
    if from_local {
        fileman::copy_files(app.clone().file_names, &conf_path, &rel_path).unwrap();
        return
    }
    fileman::copy_files(app.clone().file_names, &rel_path, &conf_path).unwrap();
}

pub fn sync_application(app_name: &str, skip_push: bool) {
    logger::print_job("Syncing application ".to_owned() + &app_name);
    let apps = &mut Apps::new();
    let app = apps.find_app_by_name(app_name);
    app_copy_action(&app, false);
    if !skip_push {
        // git push command
    }
}

pub fn sync_all_applications(skip_push: bool) {
    logger::print_job("Syncing all applications' settings".to_owned());
    let apps = Apps::new();
    for app in apps.items.iter() {
        app_copy_action(app, false);
    }
    if !skip_push {
        // git push command
    }
}

pub fn install_application(app_name: &str) {
    logger::print_job("Installing application".to_owned() + &app_name);
    let apps = &mut Apps::new();
    let app = apps.find_app_by_name(app_name);
    app_copy_action(&app, true);
}

pub fn install_all_applications() {
    logger::print_job("Installing all applications' settings".to_owned());
    let apps = Apps::new();
    for app in apps.items.iter() {
        app_copy_action(app, true);
    }
}

fn uninstall_pre(ru_sure: String, job_msg: String) -> Apps {
    let ans = are_you_sure(ru_sure, false);
    if !ans {
        logger::print_info("Exiting".to_owned());
        std::process::exit(0);
    }
    logger::print_job(job_msg);
    Apps::new()
}

pub fn uninstall_application(app_name: &str) {
    let mut apps = uninstall_pre(
        "uninstall ".to_owned() + &app_name,
        "Uninstalling ".to_owned() + &app_name);
    let app = apps.find_app_by_name(&app_name);
    let conf_path = get_absolute_path(&app.config_path);
    fileman::remove_files(&conf_path);
}

pub fn uninstall_all_applications() {
    let apps = uninstall_pre(
        "uninstall all applications' settings".to_owned(),
        "Uninstalling all applications".to_owned());
    for app in apps.items.iter() {
        let conf_path = get_absolute_path(&app.config_path);
        print_app_info(&conf_path, app.clone().file_names);
        fileman::remove_files(&conf_path);
    }
}

pub fn remove_application(app_name: &str) {
    logger::print_warn("Removing ".to_owned() + &app_name);
    let mut apps = Apps::new();
    apps.remove_app(app_name);
}
