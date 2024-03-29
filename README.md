# fix-path-env-rs
![Test](https://github.com/tauri-apps/fix-path-env-rs/workflows/Test/badge.svg)

A Rust crate to fix the `PATH` environment variable on macOS and Linux when running a GUI app.

GUI apps on macOS and Linux do not inherit the `$PATH` from your shell dotfiles (*.bashrc, .bash_profile, .zshrc, etc*).

## Installation
Please note, below in the dependencies you can also lock to a revision/tag in the `Cargo.toml`.
```toml
[dependencies]
fix-path-env = { git = "https://github.com/tauri-apps/fix-path-env-rs" }
```

## Usage
Call `fix_path_env::fix()` as early as possible in your `main` function in `main.rs` file

```rust
fn main() {
    let _ = fix_path_env::fix(); // <---- Add this
}
```

# License
MIT / Apache-2.0
