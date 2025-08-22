# fix-path-env-rs
![Test](https://github.com/tauri-apps/fix-path-env-rs/workflows/Test/badge.svg)

A Rust crate to fix the `PATH` environment variable on Windows, macOS, and Linux when running a GUI app.

GUI apps have different PATH inheritance issues on different platforms:
- **Windows**: `std::process::Command` sometimes doesn't inherit the parent process's PATH
- **macOS/Linux**: GUI apps don't inherit the `$PATH` from your shell dotfiles (*.bashrc, .bash_profile, .zshrc, etc*)

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

## Platform Behavior

### Windows
Fixes the PATH inheritance bug in `std::process::Command` where child processes sometimes can't find executables that are in PATH. This is a known issue where GUI applications on Windows don't properly inherit environment variables from their parent process.

### macOS & Linux
Reads your shell configuration (`.zshrc`, `.bash_profile`, etc.) to get the PATH that would be available in your terminal, then applies it to the current process. This ensures GUI applications can find commands installed via Homebrew, package managers, or manually added to your shell PATH.

# License
MIT / Apache-2.0
