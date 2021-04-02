extern crate clap;
extern crate colored;
extern crate toml;
extern crate serde;

use std::process::exit;
use colored::*;
use serde::Deserialize;

mod readline;
mod fileman;
mod args;

#[derive(Deserialize)]
struct Apps {
    items: Vec<fileman::Application>,
}

fn take_new_application() {
    println!("{} {}", "[-]".blue().bold(), "New application:".bold());
    let app_name = readline::read("Enter Application name: ");
    let app_config_path = readline::read("Config path (relative to home): ");
    let config_files = readline::read("File names to save (space separated): ");

    println!("App name: {}\nPath: {}\nFiles: {}", app_name, app_config_path, config_files);
}

fn sync_application() {
    println!("sync application");
}

fn sync_all_applications() {
    println!("sync all applications");
}

fn install_application() {
    println!("install application");
}

fn uninstall_application() {
    println!("uninstall application");
}

fn main() {
    let matches = args::parse_args();
    if matches.is_present("install") {
        install_application();
    } else if matches.is_present("uninstall") {
        uninstall_application();
    } else if matches.is_present("sync") {
        sync_application();
    } else if matches.is_present("sync_all") {
        sync_all_applications();
    } else if matches.is_present("new"){
        take_new_application();
    } else {
        println!("no operation specified, exiting");
        exit(0);
    }
}
