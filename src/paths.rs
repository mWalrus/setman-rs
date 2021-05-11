// Copyright (c) 2021 Oskar Hellkvist <hellkvistoskar@protonmail.com>

// SPDX-License-Identifier: BSD-2-Clause

use home::home_dir;

#[derive(Debug, Clone)]
pub struct Paths {
    pub user_conf_path: String,
    pub settings_path: String,
    pub apps_config_path: String,
    pub git_config_path: String,
}

impl Paths {
    pub fn new() -> Paths {
        Paths {
            user_conf_path: home_dir().unwrap().display().to_string() + "/.config/setman",
            settings_path: home_dir().unwrap().display().to_string() + "/.config/setman/settings",
            apps_config_path: home_dir().unwrap().display().to_string()
                + "/.config/setman/apps.toml",
            git_config_path: home_dir().unwrap().display().to_string() + "/.config/setman/git.toml",
        }
    }

    pub fn get_app_path(self, app_name: &str) -> String {
        self.settings_path + "/" + app_name
    }

    pub fn get_absolute_path(self, rel_path: &str) -> String {
        home_dir().unwrap().display().to_string() + "/" + rel_path
    }
}
