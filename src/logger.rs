extern crate colored;

use colored::*;

#[allow(dead_code)]
pub fn print_job(message: String) {
    println!("{} {}", "[~]".yellow().bold(), message);
}

pub fn print_info(message: String) {
    println!("{} {}", "[*]".blue().bold(), message);
}

pub fn print_warn(message: String) {
    println!("{} {}", "[!]".red().bold(), message);
}

pub fn print_new_app_header() {
    println!("{} {}", "[-]".blue().bold(), "New application:".bold());
}

pub fn print_app(app_name: String, app_config_path: String, app_file_names: Vec<String>, verbose: bool) {
    println!("  {} {}", "-".bold().bright_purple(), app_name.bold());
    if verbose {
        println!("      {} {}", "Config path =>".bold().bright_cyan(), app_config_path);
        println!("      {}", "File names:".bold().bright_green());
        for file in app_file_names {
            println!("          {} {}", "=>".bold().bright_green(), file);
        }
    }
}
