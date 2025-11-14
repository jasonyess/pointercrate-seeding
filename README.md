right now this can only generate random records, demons, submitters, and players

to use, put this next to your pointercrate repo (both should be in the same directory), otherwise modify the dependency paths in the Cargo.toml

then `cargo build --release` and run the binary with a path to the config file (probably `.\target\release\pointercrate-seeding "path\to\config.json"`)
