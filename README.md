# fix-path-env-rs
![Test](https://github.com/tauri-apps/fix-path-env-rs/workflows/Test/badge.svg)

A Rust crate to fix the `PATH` environment variable on macOS and Linux when running a GUI app.

GUI apps on macOS and Linux do not inherit the `$PATH` from your shell dotfiles (*.bashrc, .bash_profile, .zshrc, etc*).

## Installation
There are three general methods of installation that we can recommend.
1. Pull sources directly from Github using git tags / revision hashes (most secure, good for developement, shown below)
2. Git submodule install this repo in your tauri project and then use `file` protocol to ingest the source
3. Use crates.io and npm (easiest, and requires you to trust that our publishing pipeline worked)

Please note, below in the dependencies you can also lock to a revision/tag in the `Cargo.toml`.

`src-tauri/Cargo.toml`
```yaml


[dependencies.fix-path-env]
git = "https://github.com/tauri-apps/fix-path-env-rs"
```

Use in `src-tauri/src/main.rs`:
```rust
fn main() {
    let _ = fix_path_env::fix();
    tauri::Builder::default()
        .run(tauri::generate_context!());
}
```

# License
MIT / Apache-2.0
