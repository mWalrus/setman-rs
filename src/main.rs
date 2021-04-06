extern crate clap;
extern crate colored;
extern crate toml;
extern crate serde;
extern crate home;

mod readline;
mod fileman;
mod args;
mod logger;

use colored::*;
use fileman::{Apps, App};
use std::process::exit;

//hej jag heter ellen. jag älskar dig även fast du tycker jag är jobbig. glad smiley

fn take_new_application() {
    println!("{} {}", "[-]".blue().bold(), "New application:".bold());
    let app_name = readline::read("Enter Application name: ");
    let app_config_path = readline::read("Config path (relative to home): ");
    let config_files = readline::read("File names to save (space separated): ");

    println!("App name: {}\nPath: {}\nFiles: {}", app_name, app_config_path, config_files);

    let mut files_split: Vec<String> = Vec::new();
    for file_name in config_files.split_whitespace() {
        files_split.push(file_name.to_string());
    }
    let mut apps = Apps::new();
    apps.save_new_app((app_name, app_config_path, files_split));
}

// Gets the absolute path to a config directory
fn get_absolute_path(relative_path: &str) -> String {
    home::home_dir().unwrap().display().to_string() + "/" + relative_path + "/"
}

fn print_app_list() {
    logger::print_info("Found applications".to_owned());
    let mut apps = Apps::new();
    apps.read_apps();
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

fn sync_application(app_name: &str) {
    logger::print_job("Syncing application ".to_owned() + &app_name);
    let apps = &mut Apps::new();
    let app = apps.find_app_by_name(app_name);
    app_copy_action(&app, false);
}

fn sync_all_applications() {
    logger::print_job("Syncing all applications' settings".to_owned());
    let mut apps = Apps::new();
    apps.read_apps();
    for app in apps.items.iter() {
        app_copy_action(app, false);
    }
}

fn install_application(app_name: &str) {
    logger::print_job("Installing application".to_owned() + &app_name);
    let apps = &mut Apps::new();
    let app = apps.find_app_by_name(app_name);
    app_copy_action(&app, true);
}

fn install_all_applications() {
    logger::print_job("Installing all applications' settings".to_owned());
    let mut apps = Apps::new();
    apps.read_apps();
    for app in apps.items.iter() {
        app_copy_action(app, true);
    }
}

fn uninstall_application(app_name: &str) {
    let ans = are_you_sure("uninstall ".to_owned() + &app_name, false);
    if !ans {
        logger::print_info("Exiting".to_owned());
        return
    }
    logger::print_job("Uninstalling application ".to_owned() + &app_name);
    let mut apps = Apps::new();
    let app = apps.find_app_by_name(&app_name);
    let conf_path = get_absolute_path(&app.config_path);
    fileman::remove_files(&conf_path);
}

fn uninstall_all_applications() {
    let ans = are_you_sure("uninstall all applications' settings".to_owned(), false);
    if !ans {
        logger::print_info("Exiting".to_owned());
        return
    }
    logger::print_job("Uninstalling all applications".to_owned());
    let mut apps = Apps::new();
    apps.read_apps();
    for app in apps.items.iter() {
        let conf_path = get_absolute_path(&app.config_path);
        print_app_info(&conf_path, app.clone().file_names);
        fileman::remove_files(&conf_path);
    }
}

fn main() {
    let matches = args::parse_args();

    if matches.is_present("list") {
        print_app_list();
        exit(0);
    }

    if matches.is_present("install") {
        let app_name = matches.value_of("install").unwrap();
        install_application(app_name);
    }
    else if matches.is_present("uninstall") {
        let app_name = matches.value_of("uninstall").unwrap();
        uninstall_application(app_name);
    }
    else if matches.is_present("sync") {
        let app_name = matches.value_of("sync").unwrap();
        sync_application(app_name);
    }
    else if matches.is_present("install_all") { install_all_applications(); }
    else if matches.is_present("uninstall_all") { uninstall_all_applications(); }
    else if matches.is_present("sync_all") { sync_all_applications(); }
    else if matches.is_present("new") { take_new_application(); }
}
