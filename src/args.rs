// Copyright (c) 2021 Oskar Hellkvist <hellkvistoskar@protonmail.com>

// SPDX-License-Identifier: BSD-2-Clause

use clap::{App, Arg, ArgMatches, SubCommand};

pub fn parse_args() -> ArgMatches<'static> {
    App::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .subcommand(
            SubCommand::with_name("list")
                .about("Lists all applications")
                .arg(
                    Arg::with_name("app")
                        .help("Application to list")
                        .multiple(true),
                )
                .subcommand(SubCommand::with_name("verbose").help("Print verbose list")),
        )
        .subcommand(
            SubCommand::with_name("install")
                .about("Install settings for an application")
                .subcommand(
                    SubCommand::with_name("app")
                        .help("Application to install settings for")
                        .about("Select an application to install for")
                        .arg(
                            Arg::with_name("application")
                                .help("App to install for")
                                .takes_value(true),
                        ),
                )
                .subcommand(
                    SubCommand::with_name("all")
                        .about("Installs all added applications")
                        .arg(
                            Arg::with_name("skip")
                                .long("skip")
                                .value_name("application")
                                .multiple(true)
                                .takes_value(true)
                                .help("App to skip"),
                        ),
                ),
        )
        .subcommand(
            SubCommand::with_name("uninstall")
                .about("Uninstall settings for an application")
                .subcommand(
                    SubCommand::with_name("app")
                        .help("Application to install settings for")
                        .about("Select an application to install for")
                        .arg(
                            Arg::with_name("application")
                                .help("App to install for")
                                .takes_value(true),
                        ),
                )
                .subcommand(
                    SubCommand::with_name("all")
                        .about("Installs all added applications")
                        .arg(
                            Arg::with_name("skip")
                                .long("skip")
                                .takes_value(true)
                                .multiple(true)
                                .help("App to skip"),
                        ),
                ),
        )
        .subcommand(
            SubCommand::with_name("save")
                .about("Save settings for an application")
                .subcommand(
                    SubCommand::with_name("app")
                        .help("Application to install settings for")
                        .about("Select an application to install for")
                        .arg(
                            Arg::with_name("application")
                                .help("App to install for")
                                .takes_value(true),
                        ),
                )
                .subcommand(
                    SubCommand::with_name("all")
                        .about("Installs all added applications")
                        .arg(
                            Arg::with_name("skip")
                                .long("skip")
                                .takes_value(true)
                                .multiple(true)
                                .help("App to skip"),
                        ),
                ),
        )
        .subcommand(
            SubCommand::with_name("sync")
                .about("Sync settings")
                .arg(
                    Arg::with_name("direction")
                        .help("Chose to sync from or to remote")
                        .index(1),
                )
                .subcommand(
                    SubCommand::with_name("all")
                        .help("Perform action on all added applications")
                        .arg(
                            Arg::with_name("skip")
                                .long("skip")
                                .takes_value(true)
                                .multiple(true)
                                .help("App to skip"),
                        ),
                ),
        )
        .subcommand(SubCommand::with_name("new").about("Define a new application"))
        .subcommand(
            SubCommand::with_name("remove")
                .about("Remove a saved application")
                .arg(Arg::with_name("app").help("Application to remove").index(1)),
        )
        .subcommand(
            SubCommand::with_name("modify")
                .about("Modify an application")
                .arg(Arg::with_name("app").help("Application to modify").index(1)),
        )
        .subcommand(
            SubCommand::with_name("sync").about("Sync settings").arg(
                Arg::with_name("direction")
                    .help("Chose to sync from or to remote")
                    .index(1),
            ),
        )
        .subcommand(SubCommand::with_name("compare")
            .about("Checks if upstream has newer content than the local copy"))
        .get_matches()
}
