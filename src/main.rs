extern crate clap;
extern crate colored;
extern crate home;
extern crate toml;
extern crate serde;
extern crate git2;
extern crate uuid;
extern crate dialoguer;

mod args;
mod fileman;
mod gitman;
mod logger;
mod paths;
mod readline;
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

    match args::parse_args().subcommand() {
        ("list", Some(sub_m)) => {
            let verbose = match sub_m.subcommand() {
                ("verbose", Some(_s)) => true,
                _ => false,
            };
            let app_names = match sub_m.is_present("app") {
                true => {
                    let values = sub_m.values_of("app").unwrap();
                    Some(values.collect::<Vec<&str>>())
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
            setman::modify_application(app_name).unwrap()
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

fn perform_action(sub_command: &ArgMatches, single: Box<dyn FnOnce(&str)>, multi: Box<dyn FnOnce(Vec<&str>)>) {
    match sub_command.subcommand() {
        ("app", Some(cmd)) => {
            if cmd.is_present("application") {
                single(cmd.value_of("application").unwrap());
            }
        },
        ("all", Some(cmd)) => {
            match cmd.values_of("skip") {
                Some(app_names) => multi(app_names.collect::<Vec<&str>>()),
                None => multi(Vec::<&str>::new()),
            }
        },
        _ => exit(0),
    }
}
