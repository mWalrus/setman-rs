extern crate clap;

use clap::{App, Arg, ArgMatches, SubCommand};

pub fn parse_args() -> ArgMatches<'static> {
    App::new("Setman - Application settings manager")
        .version("1.1.0")
        .author("mWalrus")
        .about("Manages settings for various applications")
        .subcommand(SubCommand::with_name("list")
            .about("Lists all applications")
            .arg(Arg::with_name("app")
            .help("Application to list")
            .index(1))
            .subcommand(SubCommand::with_name("verbose")
                .help("Print verbose list")))
        .subcommand(SubCommand::with_name("install")
            .about("Install settings for an application")
            .arg(Arg::with_name("app")
                .help("Application to install settings for")
                .index(1)))
        .subcommand(SubCommand::with_name("uninstall")
            .about("Uninstall settings for an application")
            .arg(Arg::with_name("app")
                .help("Application to install settings for")
                .index(1)))
        .subcommand(SubCommand::with_name("save")
            .about("Save settings for an application")
            .arg(Arg::with_name("app")
                .help("Application to install settings for")
                .index(1)))
        .subcommand(SubCommand::with_name("sync")
            .about("Sync settings")
            .arg(Arg::with_name("direction")
                .help("Chose to sync from or to remote")
                .index(1)))
        .subcommand(SubCommand::with_name("new")
            .about("Define a new application"))
        .subcommand(SubCommand::with_name("remove")
            .about("Remove a saved application")
            .arg(Arg::with_name("app")
                .help("Application to remove")
                .index(1)))
         .subcommand(SubCommand::with_name("modify")
            .about("Modify an application")
            .arg(Arg::with_name("app")
                .help("Application to modify")
                .index(1)))
        .subcommand(SubCommand::with_name("sync")
            .about("Sync settings")
            .arg(Arg::with_name("direction")
                .help("Chose to sync from or to remote")
                .index(1)))
        .get_matches()
}
