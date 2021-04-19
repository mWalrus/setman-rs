# SetMan - Application settings manager

Traditionally when you have multiple devices and you want to have the same configs for your favorite apps you would use github or some other cloud storage manage your configs. This can become tedious work to keep track of when your config repo gets larger and managing your configs gets harder than it once was just because of the sheer size of the repository.
SetMan solves this problem! It works by letting you enter the application you want to manage, it's config path and the files you want SetMan to track.
When you tell SetMan to sync your configs to github it will create a temporary clone of your git repo, copy the local files to it and push the changes to your upstream repo.

## Dependencies
- Rust
- Cargo

## Installation
1. `git clone https://github.com/mWalrus/setman-rs.git setman`
2. `cd setman`
3. `cargo build --release`
4. `sudo install -s -Dm755 ./target/release/setman-rs /usr/bin/setman`
