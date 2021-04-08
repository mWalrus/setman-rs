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
