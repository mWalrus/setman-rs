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

use std::process::exit;

use fileman::{Apps, App};
use gitman::GitRepo;
use colored::*;

static _LOCAL_CONF_PATH: &str = ".config/setman/";
static LOCAL_SETTINGS_PATH: &str = ".config/setman/settings/";

pub fn sync_settings(direction: &str) {
    let mut gitman = GitRepo::new();
    gitman.clone_repo();
    let repo_path = gitman.get_repo_path();
    if direction.eq("down") {
        let dirs_to_copy = gitman.clone().get_dir_names();
        fileman::copy_files(
            dirs_to_copy.to_owned(),
            repo_path,
            &get_absolute_path(LOCAL_SETTINGS_PATH)
        ).unwrap();
        return
    }
    // otherwise copy from local folder into git folder
    let apps: Apps = Apps::new();
    let app_names: Vec<String> = apps.items.into_iter().map(|app| app.name).collect();
    fileman::copy_files(app_names, LOCAL_SETTINGS_PATH, repo_path).unwrap();
    let commit_msg = readline::read("Enter a commit message");
}

pub fn take_new_application() {
    println!("{} {}", "[-]".blue().bold(), "New application:".bold());
    let app_name = readline::read("Enter Application name");
    let app_config_path = readline::read("Config path (relative to home)");
    let config_files = readline::read("File names to save (space separated)");

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
    let ans = readline::read(&format!("Are you sure you want to {}? ({})", &action, formatted));
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
    return (
        get_absolute_path(&app.config_path),
        get_absolute_path(&(LOCAL_SETTINGS_PATH.to_string() + &app.name))
    )
}

fn app_copy_action(app: &App, from_local: bool) {
    let (app_conf_path, local_path) = get_paths(app);
    print_app_info(&app_conf_path, app.clone().file_names);
    if from_local {
        fileman::copy_files(app.clone().file_names, &app_conf_path, &local_path).unwrap();
        return
    }
    fileman::copy_files(app.clone().file_names, &local_path, &app_conf_path).unwrap();
}

pub fn save_application(app_name: &str) {
    logger::print_job("Saving application ".to_owned() + &app_name + " to local collection");
    let apps = &mut Apps::new();
    let app = apps.find_app_by_name(app_name);
    app_copy_action(&app, false);
}

pub fn save_all_applications() {
    logger::print_job("Saving all applications' settings".to_owned());
    let apps = Apps::new();
    for app in apps.items.iter() {
        app_copy_action(app, false);
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
        exit(0);
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

fn exit_on_invalid() {
    logger::print_warn("Invalid option, exiting.".to_owned());
    exit(0);
}

pub fn modify_application(app_name: &str) {
    let mut apps = Apps::new();
    let mut app = apps.find_app_by_name(&app_name);
    logger::print_info("Modify ".to_owned() + &app_name);
    println!("    {} Name\n    {} Config path\n    {} File names", "1.".bold(), "2.".bold(), "3.".bold());
    let ans: i32 = readline::read("Select field you want to edit").parse().unwrap_or(-1);
    match ans {
        1 => app.name = readline::read("Enter a new name"),
        2 => app.config_path = readline::read("Enter a new config path"),
        3 => {
            let mut file_names = app.file_names.clone();
            for (i, name) in file_names.iter().enumerate() {
                println!("    {} {}", ((i+1).to_string() + ".").bold(), name);
            }
            let file_index: usize = readline::read("Select file name you want to edit").parse().unwrap_or(usize::MIN);
            // handle invalid option
            if file_index.eq(&usize::MIN) || !(0..file_names.len()).contains(&(file_index - 1)) {exit_on_invalid()};

            let new_file_name = readline::read("Enter a new file name");
            file_names.remove(file_index - 1);
            file_names.insert(file_index - 1, new_file_name);
            app.file_names = file_names;

        },
        _ => exit_on_invalid(),
    }
    // make sure user wants to modify the application
    if are_you_sure("modify ".to_owned() + &app_name, true) {
        apps.remove_app(app_name);
        apps.save_new_app(app);
    }
}
