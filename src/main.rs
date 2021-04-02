extern crate clap;
extern crate colored;
extern crate toml;
extern crate serde;

mod readline;
mod fileman;
mod args;

use std::process::exit;
use colored::*;
use serde::Deserialize;

fn take_new_application() {
    println!("{} {}", "[-]".blue().bold(), "New application:".bold());
    let app_name = readline::read("Enter Application name: ");
    let app_config_path = readline::read("Config path (relative to home): ");
    let config_files = readline::read("File names to save (space separated): ");

    println!("App name: {}\nPath: {}\nFiles: {}", app_name, app_config_path, config_files);
}

fn sync_application(app_name: &str) {
    println!("sync application");
    let mut a = fileman::Apps::new();
    let app = a.find_app_by_name(app_name);
    println!("{}", app.config_path);
}

fn sync_all_applications() {
    println!("sync all applications");
}

fn install_application(app_name: &str) {
    println!("install application");

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
