// this file will be used to clone and push settings to a specified upsteam url using git

// When a user is using the app, they should be given the option to fetch their configs from their upstream url

// This url can be set using the --set-upstream option, if upstream isnt set when the user is using the app
// the application will complain.

// When the upstream is set the user can chose to sync their settings, either clone from upstream git url or
// push their local changes to git

// git management will be separate from the local settings directory
// The local folder will be in setman's .config directory.
// When we are doing git clones or pushes we create a temporary git repo in /tmp
// this is done to remove the need for a permanent git folder in the .config directory

// I might think about redoing the sync command. So instead of a general sync command i could
// do two separate commands. One for push syncs and one for clone syncs. They should probably still
// be application specific with the option to sync all

// i will need to think more about the above thoughts

// The clone sync flag could have application specific syncing so when the repo is cloned
// setman will check if an application was specified and only copy the cloned files for that application
// to the local collection

// Same thing could be done for the push command, an option to sync all will be avaliable but application specific
// push syncs is also possible, application settings will be copied to local and pushed.

// The option to only sync a single application both ways is good if you have unfinished versions of a config you
// are not sure if you want to use or discard on another system or user.

// expand this to be able to use ssh for clone and push
// (repository struct does not support this but RepoBuilder does)

extern crate git2;
extern crate uuid;
extern crate toml;
extern crate serde;

#[path = "fileman.rs"]
mod fileman;
#[path = "readline.rs"]
mod readline;

use git2::Repository;
use uuid::Uuid;
use std::{borrow::Borrow, fs};
use std::process::exit;
use serde::{Deserialize};

static GIT_FILE: &str = "git.toml";

#[derive(Deserialize)]
pub struct GitRepo {
    upstream_url: String,
    git_dir_name: String,
}

impl GitRepo {
    pub fn new() -> GitRepo {
        let file_content = fs::read_to_string(GIT_FILE).unwrap();
        let parsed: GitRepo = toml::from_str(&file_content).unwrap();
        GitRepo {
            upstream_url: parsed.upstream_url,
            git_dir_name: Uuid::new_v4().to_string(),
        }
    }
    pub fn push_settings(self) {
        let sudo_pass = readline::read("Enter sudo password: ");
        let repo = Repository::clone(
            self.upstream_url.borrow(),
            &("/tmp".to_string() + self.git_dir_name.borrow()));

    }
}
