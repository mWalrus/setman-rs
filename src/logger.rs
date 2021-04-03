extern crate colored;

use colored::*;

pub fn print_job(message: String) {
    println!("{} {}", "[~]".yellow().bold(), message);
}

pub fn print_info(message: String) {
    println!("{} {}", "[*]".blue().bold(), message);
}
