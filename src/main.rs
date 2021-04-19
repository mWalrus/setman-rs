#![allow(dead_code)]
extern crate clap;
extern crate colored;

mod args;
mod setman;

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
        ("list", Some(_sub_m)) => {
            setman::print_app_list();
            exit(0);
        },
        ("install", Some(sub_m)) => {
            let app_name = sub_m.value_of("app").unwrap();
            if app_name.eq("all") {
                setman::install_all_applications();
                exit(0);
            }
            setman::install_application(&app_name);
            },
        ("uninstall", Some(sub_m)) =>  {
            let app_name = sub_m.value_of("app").unwrap();
            if app_name.eq("all") {
                setman::uninstall_all_applications();
                exit(0);
            }
            setman::uninstall_application(&app_name);
            },
        ("save", Some(sub_m)) =>  {
            let app_name = sub_m.value_of("app").unwrap();
            if app_name.eq("all") {
                setman::save_all_applications();
                exit(0);
            }
            setman::save_application(&app_name);
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
