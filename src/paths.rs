// Copyright (c) 2021 Oskar Hellkvist <hellkvistoskar@protonmail.com>

// SPDX-License-Identifier: BSD-2-Clause

use std::path::PathBuf;

use home::home_dir;

#[derive(Debug, Clone)]
pub struct Paths {
    pub setman_path: PathBuf,
    pub settings_path: PathBuf,
    pub applist_path: PathBuf,
    pub upstream_path: PathBuf,
    pub commit_id_path: PathBuf,
}

impl Paths {
    pub fn new() -> Paths {
        let home_path = home_dir().unwrap().display().to_string();

        let mut setman_path: PathBuf = PathBuf::from(home_path);
        setman_path.push(".config/setman");

        let mut settings_path = setman_path.clone();
        settings_path.push("settings");

        let mut applist_path = setman_path.clone();
        applist_path.push("apps.toml");

        let mut upstream_path = setman_path.clone();
        upstream_path.push("upstream_url");

        let mut commit_id_path = setman_path.clone();
        commit_id_path.push("latest_commit");

        Paths {
            setman_path,
            settings_path,
            applist_path,
            upstream_path,
            commit_id_path
        }
    }

    pub fn get_app_path(self, app_name: &str) -> PathBuf {
        let mut path = self.settings_path.clone();
        path.push(app_name);
        path
    }

    pub fn get_absolute_path(self, rel_path: &str) -> PathBuf {
        let home = home_dir().unwrap().display().to_string();
        let mut path = PathBuf::from(home);
        path.push(rel_path);
        path
    }
}
