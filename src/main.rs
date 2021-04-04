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
use fileman::{Application, Apps};

//hej jag heter ellen. jag älskar dig även fast du tycker jag är jobbig. glad smiley

fn take_new_application() {
    println!("{} {}", "[-]".blue().bold(), "New application:".bold());
    let app_name = readline::read("Enter Application name: ");
    let app_config_path = readline::read("Config path (relative to home): ");
    let config_files = readline::read("File names to save (space separated): ");

    println!("App name: {}\nPath: {}\nFiles: {}", app_name, app_config_path, config_files);
}

fn get_absolute_path(relative_path: &str) -> String {
    home::home_dir().unwrap().display().to_string() + "/" + relative_path + "/"
}

fn sync_application(app_name: &str) {
    logger::print_job("Syncing application ".to_owned() + &app_name);
    let apps = &mut Apps::new();
    let app = apps.find_app_by_name(&app_name);
    let conf_path = get_absolute_path(&app.config_path);
    let rel_path = "./".to_string() + &app.name + "/";
    apps.copy_files(&app, &conf_path, &rel_path);
}

fn sync_all_applications() {
    println!("sync all applications");
    let mut apps = Apps::new();
    apps.get_apps();
    let app_list = &mut apps.apps;
    for app in app_list {
        let conf_path = get_absolute_path(&app.config_path);
        let rel_path = "./".to_string() + &app.name + "/";
        apps.copy_files(app, &conf_path, &rel_path);
    }
}

fn install_application(app_name: &str) {
    println!("install application");
    let mut apps = fileman::Apps::new();
    let app = apps.find_app_by_name(&app_name);
    let conf_path = get_absolute_path(&app.config_path);
    let rel_path = "./".to_string() + &app.name + "/";
    apps.copy_files(&app, &conf_path, &rel_path);

}

fn install_all_applications() {
    println!("install all");
}

fn uninstall_application(app_name: &str) {
    println!("uninstall application");
}

fn uninstall_all_applications() {
    println!("Uninstall all");
}

fn main() {
    let matches = args::parse_args();
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
