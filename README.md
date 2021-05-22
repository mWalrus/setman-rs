# SetMan - Application settings manager

A minimal application to manage dotfiles you want to sync between devices.

## Dependencies
- Rust
- Cargo

## Prerequisites
To build and install this application you will need to install `cargo-make` using cargo
```
cargo install --force cargo-make
```

## Installation
1. `git clone https://github.com/mWalrus/setman-rs.git setman`
2. `cd setman`
3. `cargo make install`

## Post install
Before you start using any of the git features of setman you will need to set up an upstream repository.
This repository can be either private or public.
You will also need to enter your git username, email and upstream url in the `git.toml` file in `$HOME/.config/setman/`

```
upstream_url = "https://github.com/username/setman-settings-repo-name.git"
name = "your-github-username"
email = "your@github.email"
pass = "giTHuB-P@$$woRd" // can be omitted
```
If you chose to omit the password field you will be prompted for it during runtime when using any of the git related functionalities.

## Usage
After the setup process is complete you can run `setman help` to view the help page for setman.
