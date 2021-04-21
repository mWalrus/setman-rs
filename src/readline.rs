use std::io::Error;

use colored::*;
use dialoguer::{Confirm, Input, Select, Password};

pub fn read(message: &str) -> Result<String, Error> {
    Ok(Input::<String>::new()
        .with_prompt(format!("{} {}", "[<]".bold().bright_cyan(), message))
        .interact_text()?.to_string())
}

pub fn password(message: &str) -> Result<String, Error> {
    Ok(Password::new()
        .with_prompt(format!("{} {}", "[<]".bold().bright_cyan(), message))
        .interact()?.to_string())
}

pub fn select(items: Vec<&str>) -> Result<usize, Error> {
    Ok(Select::new().items(&items).interact()?)
}

pub fn are_you_sure(action: String) -> Result<bool, Error> {
    Ok(Confirm::new()
        .with_prompt(format!("{} Are you sure you want to {}?", "[?]".bold().green(), action))
        .interact()?)
}
