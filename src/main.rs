// Copyright (c) 2021 Oskar Hellkvist <hellkvistoskar@protonmail.com>

// SPDX-License-Identifier: BSD-2-Clause

extern crate clap;
extern crate colored;
extern crate dialoguer;
extern crate git2;
extern crate home;
extern crate serde;
extern crate toml;
extern crate uuid;
extern crate regex;
extern crate thiserror;

mod args;
mod fileman;
mod gitman;
mod logger;
mod paths;
mod readline;
mod setman;

use clap::Values;
use setman::SetmanAction;
use setman::ListOptions;

//hej jag heter ellen. jag älskar dig även fast du tycker jag är jobbig. glad smiley

fn main() {
    logger::print_header();

    match args::parse_args().subcommand() {
        ("list", Some(sub_m)) => {
            let verbose = match sub_m.subcommand() {
                ("verbose", Some(_s)) => true,
                _ => false,
            };

            let regex = match sub_m.is_present("regex") {
                true => Some(sub_m.value_of("regex").unwrap()),
                false => None,
            };
            if regex.ne(&None) {
                setman::print_app_list(ListOptions::Regex(regex.unwrap()), verbose);
                return;
            }

            let app_names = match sub_m.is_present("app") {
                true => {
                    let values = sub_m.values_of("app").unwrap();
                    Some(values.collect::<Vec<&str>>())
                }
                false => None,
            };
            setman::print_app_list(ListOptions::Literal(&app_names), verbose);
        }
        ("install", Some(sub_m)) => {
            match sub_m.subcommand() {
                ("app", Some(app_subcommand)) => {
                    setman::app_action(
                        SetmanAction::Install(
                            app_subcommand.value_of("application").unwrap()
                        )
                    );
                },
                ("all", Some(all_subcommand)) => {
                    setman::all_apps_action(
                        SetmanAction::InstallAll(
                            &get_skipped_apps(all_subcommand.values_of("skip"))
                        )
                    )
                },
                _ => {}
            }
        },
        ("uninstall", Some(sub_m)) => {
            match sub_m.subcommand() {
                ("app", Some(app_subcommand)) => {
                    setman::app_action(
                        SetmanAction::Uninstall(
                            app_subcommand.value_of("application").unwrap()
                        )
                    );
                },
                ("all", Some(all_subcommand)) => {
                    setman::all_apps_action(
                        SetmanAction::UninstallAll(
                            &get_skipped_apps(all_subcommand.values_of("skip"))
                        )
                    );
                },
                _ => {}
            }
        },
        ("save", Some(sub_m)) => {
            match sub_m.subcommand() {
                ("app", Some(app_subcommand)) => {
                    setman::app_action(
                        SetmanAction::Save(
                            app_subcommand.value_of("application").unwrap()
                        )
                    );
                },
                ("all", Some(all_subcommand)) => {
                    setman::all_apps_action(
                        SetmanAction::SaveAll(
                            &get_skipped_apps(all_subcommand.values_of("skip"))
                        )
                    );
                },
                _ => {}
            };
            if sub_m.is_present("push") {
                setman::sync_settings(SetmanAction::SyncUp);
            }
        },
        ("modify", Some(sub_m)) => {
            let app_name = sub_m.value_of("app").unwrap();
            setman::app_action(SetmanAction::Modify(&app_name));
        }
        ("remove", Some(sub_m)) => {
            let app_name = sub_m.value_of("app").unwrap();
            setman::app_action(SetmanAction::Remove(&app_name));
        }
        ("new", Some(_sub_m)) => setman::app_action(SetmanAction::New),
        ("sync", Some(sub_m)) => {
            let direction = sub_m
                .value_of("direction")
                .unwrap()
                .to_lowercase();
            match direction.eq("up") {
                true => setman::sync_settings(SetmanAction::SyncUp),
                false => setman::sync_settings(SetmanAction::SyncDown),
            };
        },
        ("compare", Some(_sub_m)) => {
            setman::compare_upstream();
        },
        _ => panic!("Invalid option"),
    }
}

fn get_skipped_apps<'a>(arg_values: Option<Values<'a>>) -> Vec<String> {
    match arg_values {
        Some(app_names) => {
            app_names
                .map(|names| names.to_string())
                .collect::<Vec<String>>()
        },
        None => vec![],
    }
}
