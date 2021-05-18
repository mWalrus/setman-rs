// Copyright (c) 2021 Oskar Hellkvist <hellkvistoskar@protonmail.com>

// SPDX-License-Identifier: BSD-2-Clause

use std::path::PathBuf;

use colored::*;

pub fn print_header() {
    println!(
        "{}\n\n{}\n",
        "      ::::::::  :::::::::: ::::::::::: :::   :::       :::     ::::    :::
    :+:    :+: :+:            :+:    :+:+: :+:+:    :+: :+:   :+:+:   :+:
   +:+        +:+            +:+   +:+ +:+:+ +:+  +:+   +:+  :+:+:+  +:+
  +#++:++#++ +#++:++#       +#+   +#+  +:+  +#+ +#++:++#++: +#+ +:+ +#+
        +#+ +#+            +#+   +#+       +#+ +#+     +#+ +#+  +#+#+#
#+#    #+# #+#            #+#   #+#       #+# #+#     #+# #+#   #+#+#
########  ##########     ###   ###       ### ###     ### ###    ####"
            .bold()
            .blue(),
        "Application settings manager".bright_cyan().bold()
    );
}

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

pub fn print_app(
    app_name: String,
    app_config_path: PathBuf,
    app_file_names: Vec<String>,
    verbose: bool,
) {
    print_info(app_name);
    if verbose {
        println!(
            "{:>18} {:?}",
            "Config path =>".bold().bright_cyan(),
            app_config_path
        );
        println!("{:>15}", "File names:".bold().bright_green());
        for file in app_file_names {
            println!("{:>11} {}", "=>".bold().bright_green(), file);
        }
    }
}
