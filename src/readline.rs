extern crate rustyline;
extern crate colored;
extern crate dialoguer;

use colored::*;
use dialoguer::{Confirm, Input, Select, Password};

pub fn read(message: &str) -> String {
    Input::<String>::new()
        .with_prompt(format!("{} {}", "[<]".bold().bright_cyan(), message))
        .interact_text().unwrap().to_string()
}

pub fn password(message: &str) -> String {
    Password::new()
        .with_prompt(format!("{} {}", "[<]".bold().bright_cyan(), message))
        .interact().unwrap().to_string()
}

pub fn select(items: Vec<&str>) -> usize {
    Select::new().items(&items).interact().unwrap()
}

pub fn are_you_sure(action: String) -> bool {
    Confirm::new()
        .with_prompt(format!("{} Are you sure you want to {}?", "[?]".bold().green(), action))
        .interact().unwrap()
}
