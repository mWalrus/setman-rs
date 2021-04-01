extern crate clap;
extern crate colored;
extern crate toml;
extern crate serde;

use std::process::exit;
use clap::{Arg, App};
use colored::*;
use serde::{Deserialize};

mod readline;
mod fileman;

#[derive(Deserialize)]
struct Apps {
    items: Vec<fileman::Application>,
}

fn take_new_application() {
    println!("{} {}", "[-]".blue().bold(), "New application:".bold());
    let app_name = readline::read("Enter Application name: ");
    let app_config_path = readline::read("Config path (absolute): ");
    let config_files = readline::read("Config files to save (file name, space separated): ");

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
    let matches = App::new("Setman - Application settings manager")
        .version("0.1")
        .author("mWalrus")
        .about("Manages settings for various applications")
        .arg(Arg::with_name("install")
            .short("i")
            .long("install")
            .help("Installs settings")
            .takes_value(false))
        .arg(Arg::with_name("uninstall")
            .short("u")
            .long("uninstall")
            .help("Uninstalls settings")
            .takes_value(false))
        .arg(Arg::with_name("sync")
            .short("s")
            .long("sync")
            .help("Synchronizes settings")
            .takes_value(false))
        .arg(Arg::with_name("application")
            .short("a")
            .long("application")
            .value_name("APPLICATION")
            .help("Application which settings should be installed for")
            .takes_value(true)
            .required_unless("new"))
        .arg(Arg::with_name("new")
            .short("n")
            .long("new")
            .help("Add a new application"))
        .arg(Arg::with_name("config")
            .short("p")
            .long("path")
            .value_name("CONFIG_PATH")
            .help("Custom settings root path for specified application")
            .takes_value(true))
        .get_matches();

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
