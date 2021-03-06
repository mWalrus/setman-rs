// Copyright (c) 2021 Oskar Hellkvist <hellkvistoskar@protonmail.com>

// SPDX-License-Identifier: BSD-2-Clause

extern crate clap;
extern crate colored;
extern crate dialoguer;
extern crate git2;
extern crate home;
extern crate regex;
extern crate serde;
extern crate thiserror;
extern crate toml;
extern crate uuid;

#[macro_use]
mod logger;
mod args;
mod fileman;
mod gitman;
mod paths;
mod readline;
mod setman;

use clap::Values;
use setman::ListOptions;
use setman::SetManAction;

//hej jag heter ellen. jag älskar dig även fast du tycker jag är jobbig. glad smiley

pub enum LogLevel {
    Job,
    Info,
    Warning,
}

fn main() {
    logger::print_header();

    match args::parse_args().subcommand() {
        ("list", Some(sub_m)) => {
            let verbose = matches!(sub_m.subcommand(), ("verbose", Some(_s)));

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
        ("install", Some(sub_m)) => match sub_m.subcommand() {
            ("app", Some(app_subcommand)) => {
                setman::app_action(SetManAction::Install(
                    app_subcommand.value_of("application").unwrap(),
                ));
            }
            ("all", Some(all_subcommand)) => {
                job!("Installing all applications");
                setman::all_apps_action(SetManAction::InstallAll(&get_skipped_apps(
                    all_subcommand.values_of("skip"),
                )))
            }
            _ => {}
        },
        ("uninstall", Some(sub_m)) => match sub_m.subcommand() {
            ("app", Some(app_subcommand)) => {
                setman::app_action(SetManAction::Uninstall(
                    app_subcommand.value_of("application").unwrap(),
                ));
            }
            ("all", Some(all_subcommand)) => {
                job!("Uninstalling all applications");
                setman::all_apps_action(SetManAction::UninstallAll(&get_skipped_apps(
                    all_subcommand.values_of("skip"),
                )));
            }
            _ => {}
        },
        ("save", Some(sub_m)) => {
            match sub_m.subcommand() {
                ("app", Some(app_subcommand)) => {
                    setman::app_action(SetManAction::Save(
                        app_subcommand.value_of("application").unwrap(),
                    ));
                }
                ("all", Some(all_subcommand)) => {
                    job!("Saving all applications");
                    setman::all_apps_action(SetManAction::SaveAll(&get_skipped_apps(
                        all_subcommand.values_of("skip"),
                    )));
                }
                _ => {}
            };
            if sub_m.is_present("push") {
                setman::sync_settings(SetManAction::Push).unwrap();
            }
        }
        ("modify", Some(sub_m)) => {
            let app_name = sub_m.value_of("app").unwrap();
            setman::app_action(SetManAction::Modify(&app_name));
        }
        ("remove", Some(sub_m)) => {
            let app_name = sub_m.value_of("app").unwrap();
            setman::app_action(SetManAction::Remove(&app_name));
        }
        ("new", Some(_sub_m)) => setman::app_action(SetManAction::New),
        ("push", Some(_sub_m)) => setman::sync_settings(SetManAction::Push).unwrap(),
        ("pull", Some(_sub_m)) => setman::sync_settings(SetManAction::Pull).unwrap(),
        ("compare", Some(_sub_m)) => {
            setman::compare_upstream();
        }
        _ => panic!("Invalid option"),
    }
}

fn get_skipped_apps(arg_values: Option<Values<'_>>) -> Vec<String> {
    match arg_values {
        Some(app_names) => app_names
            .map(|names| names.to_string())
            .collect::<Vec<String>>(),
        None => vec![],
    }
}
