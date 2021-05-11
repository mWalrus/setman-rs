# SetMan - Application settings manager

Traditionally when you have multiple devices and you want to have the same configs for your favorite apps you would use github or some other cloud storage manage your configs. This can become tedious work to keep track of when your config repo gets larger and managing your configs gets harder than it once was just because of the sheer size of the repository.
SetMan solves this problem! It works by letting you enter the application you want to manage, it's config path and the files you want SetMan to track.

## Dependencies
- Rust
- Cargo

## Installation
1. `git clone https://github.com/mWalrus/setman-rs.git setman`
2. `cd setman`
3. `cargo build --release`
4. `sudo install -s -Dm755 ./target/release/setman-rs /usr/bin/setman`

## Post install
Before you can start using the git features of the application you need to enter your git username and upstream url in the git.toml file in $HOME/.config/setman/

```
upstream_url = "https://github.com/username/setman-settings-repo-name.git"
name = "your-github-username"
email = "your@github.email"
pass = "giTHuB-P@$$woRd" // can be omitted
```
If you chose to omit the password field you will be prompted for it during runtime when using any of the git related functionalities.

## Usage
After installing setman you can use `setman help` to view the application's options and commands.

You can also use `setman help <sub-command>` to view the help page for the given sub-command.
