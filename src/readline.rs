// Copyright (c) 2021 Oskar Hellkvist <hellkvistoskar@protonmail.com>

// SPDX-License-Identifier: BSD-2-Clause

use std::io::Error;

use colored::*;
use dialoguer::{Confirm, Input, Password, Select};

pub fn read(message: &str) -> Result<String, Error> {
    Input::<String>::new()
        .with_prompt(format!("{} {}", "[<]".bold().bright_cyan(), message))
        .interact_text()
}

pub fn select(items: Vec<&str>) -> Result<usize, Error> {
    Select::new().items(&items).interact()
}

pub fn are_you_sure(action: String) -> Result<bool, Error> {
    Confirm::new()
        .with_prompt(format!(
            "{} Are you sure you want to {}?",
            "[?]".bold().green(),
            action
        ))
        .interact()
}
