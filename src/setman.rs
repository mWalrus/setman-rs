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
#[path = "config.rs"]
mod config;

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
    apps.save_new_app(App::new(app_name, app_config_path, files_names));
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

// FIXME: handle Err on options that cant be parsed into number
pub fn modify_application(app_name: &str) {
    let mut apps = Apps::new();
    let mut app = apps.find_app_by_name(&app_name);
    logger::print_info("Modify ".to_owned() + &app_name);
    println!("    {} Name\n    {} Config path\n    {} File names", "1.".bold(), "2.".bold(), "3.".bold());
    let ans: i32 = readline::read("Select number of field you want to edit: ").parse::<i32>().unwrap();
    match ans {
        1 => app.name = readline::read("Enter a new name: "),
        2 => app.config_path = readline::read("Enter a new config path: "),
        3 => {
            let mut file_names = app.file_names.clone();
            for (i, name) in file_names.iter().enumerate() {
                println!("    {} {}", ((i+1).to_string() + ".").bold(), name);
            }
            let file_index: usize = readline::read("Select file name you want to edit: ").parse::<usize>().unwrap();
            if !(0..file_names.len()).contains(&(file_index - 1)) {

            }
            let new_file_name = readline::read("Enter a new file name: ");
            file_names.remove(file_index - 1);
            file_names.insert(file_index - 1, new_file_name);
            app.file_names = file_names;

        },
        _ => {
            logger::print_warn("Invalid option, exiting.".to_owned());
            std::process::exit(0);
        }
    }
    println!("{:#?}", app);
    apps.remove_app(app_name);
    apps.save_new_app(app);
}
