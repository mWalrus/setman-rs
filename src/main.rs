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
    let matches = args::parse_args();

    if matches.is_present("list") {
        setman::print_app_list();
        exit(0);
    }
    if matches.is_present("install") {
        let app_name = matches.value_of("install").unwrap();
        setman::install_application(app_name);
        exit(0);
    }
    if matches.is_present("uninstall") {
        let app_name = matches.value_of("uninstall").unwrap();
        setman::uninstall_application(app_name);
        exit(0);
    }
    if matches.is_present("sync") {
        let app_name = matches.value_of("sync").unwrap();
        setman::sync_application(app_name, matches.is_present("skip_push"));
        exit(0);
    }
    if matches.is_present("install_all") {
        setman::install_all_applications();
        exit(0);
    }
    if matches.is_present("uninstall_all") {
        setman::uninstall_all_applications();
        exit(0);
    }
    if matches.is_present("sync_all") {
        setman::sync_all_applications(matches.is_present("skip_push"));
        exit(0);
    }
    if matches.is_present("new") {
        setman::take_new_application();
        exit(0);
    }
    if matches.is_present("remove_app") {
        let app_name = matches.value_of("remove_app").unwrap();
        setman::remove_application(app_name);
    }
}



