#![allow(dead_code)]
extern crate clap;
extern crate colored;

mod args;
mod setman;

use clap::ArgMatches;
use colored::*;
use std::process::exit;

//hej jag heter ellen. jag älskar dig även fast du tycker jag är jobbig. glad smiley

fn main() {
    println!("{}\n\n{}\n", "      ::::::::  :::::::::: ::::::::::: :::   :::       :::     ::::    :::
    :+:    :+: :+:            :+:    :+:+: :+:+:    :+: :+:   :+:+:   :+:
   +:+        +:+            +:+   +:+ +:+:+ +:+  +:+   +:+  :+:+:+  +:+
  +#++:++#++ +#++:++#       +#+   +#+  +:+  +#+ +#++:++#++: +#+ +:+ +#+
        +#+ +#+            +#+   +#+       +#+ +#+     +#+ +#+  +#+#+#
#+#    #+# #+#            #+#   #+#       #+# #+#     #+# #+#   #+#+#
########  ##########     ###   ###       ### ###     ### ###    ####       ".bold().blue(), "Application settings manager".bright_cyan().bold());
    setman::check_path_existance();

    let matches = args::parse_args();

    match matches.subcommand() {
        ("list", Some(sub_m)) => {
            let verbose = match sub_m.subcommand() {
                ("verbose", Some(_s)) => true,
                _ => false,
            };
            let app_names = match sub_m.is_present("app") {
                true => {
                    let values = sub_m.values_of("app").unwrap();
                    let result: Vec<&str> = values.into_iter().map(|value| value).collect();
                    Some(result)
                },
                false => None,
            };

            setman::print_app_list(app_names, verbose);
        },
        ("install", Some(sub_m)) => {
            perform_action(
                sub_m,
                Box::new(setman::install_application),
                Box::new(setman::install_all_applications))
        },
        ("uninstall", Some(sub_m)) =>  {
            perform_action(
                sub_m,
                Box::new(setman::uninstall_application),
                Box::new(setman::uninstall_all_applications))
        },
        ("save", Some(sub_m)) =>  {
            perform_action(
                sub_m,
                Box::new(setman::save_application),
                Box::new(setman::save_all_applications))
        },
        ("modify", Some(sub_m)) =>  {
            let app_name = sub_m.value_of("app").unwrap();
            setman::modify_application(app_name)
        },
        ("remove", Some(sub_m)) =>  {
            let app_name = sub_m.value_of("app").unwrap();
            setman::remove_application(&app_name);
        },
        ("new", Some(_sub_m)) => setman::take_new_application(),
        ("sync", Some(sub_m)) => {
            let direction = sub_m.value_of("direction").unwrap().to_lowercase();
            setman::sync_settings(&direction);
        },
        _ => exit(0),
    }
}

fn perform_action(sub_command: &ArgMatches, single: Box<dyn FnOnce(&str)>, multi: Box<dyn FnOnce()>) {
    let app_name = sub_command.value_of("app").unwrap();
    if app_name.eq("all") {
        multi();
        exit(0);
    }
    single(&app_name);
}
