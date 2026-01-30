// Copyright 2021 Tauri Programme within The Commons Conservancy
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

use std::collections::HashMap;

/// The error that might happen on a [`fix`] call.
#[derive(Debug, thiserror::Error)]
pub enum Error {
  #[error(transparent)]
  Shell(#[from] std::io::Error),
  #[error("invalid output from shell echo: {0}")]
  InvalidOutput(String),
  #[error("failed to run shell echo: {0}")]
  EchoFailed(String),
}

/// Returns specified environment variables from the login shell without setting them.
///
/// If `vars` is empty, returns all environment variables from the shell.
///
/// ## Platform-specific
///
/// - **Windows**: Returns current env var values (workaround for std::process::Command bug)
/// - **macOS/Linux**: Reads shell configuration from login shell
pub fn get_vars(vars: &[&str]) -> std::result::Result<HashMap<String, String>, Error> {
  #[cfg(windows)]
  {
    let mut result = HashMap::new();
    if vars.is_empty() {
      for (key, value) in std::env::vars() {
        result.insert(key, value);
      }
    } else {
      for &var in vars {
        if let Ok(value) = std::env::var(var) {
          result.insert(var.to_string(), value);
        }
      }
    }
    Ok(result)
  }
  #[cfg(not(windows))]
  {
    let default_shell = if cfg!(target_os = "macos") {
      "/bin/zsh"
    } else {
      "/bin/sh"
    };
    let shell = std::env::var("SHELL").unwrap_or_else(|_| default_shell.into());

    let mut cmd = std::process::Command::new(shell);

    cmd
      .arg("-ilc")
      .arg("echo -n \"_SHELL_ENV_DELIMITER_\"; env; echo -n \"_SHELL_ENV_DELIMITER_\"; exit")
      // Disables Oh My Zsh auto-update thing that can block the process.
      .env("DISABLE_AUTO_UPDATE", "true");

    if let Some(home) = home::home_dir() {
      cmd.current_dir(home);
    }

    let out = cmd.output().map_err(Error::Shell)?;

    if out.status.success() {
      let stdout = String::from_utf8_lossy(&out.stdout).into_owned();
      let env = stdout
        .split("_SHELL_ENV_DELIMITER_")
        .nth(1)
        .ok_or_else(|| Error::InvalidOutput(stdout.clone()))?;

      let mut result = HashMap::new();
      for line in String::from_utf8_lossy(&strip_ansi_escapes::strip(env))
        .split('\n')
        .filter(|l| !l.is_empty())
      {
        let mut s = line.splitn(2, '=');
        if let (Some(var), Some(value)) = (s.next(), s.next()) {
          if vars.is_empty() || vars.contains(&var) {
            result.insert(var.to_string(), value.to_string());
          }
        }
      }
      Ok(result)
    } else {
      Err(Error::EchoFailed(
        String::from_utf8_lossy(&out.stderr).into_owned(),
      ))
    }
  }
}

/// Returns all environment variables from the login shell without setting them.
///
/// ## Platform-specific
///
/// - **Windows**: Returns all current env vars (workaround for std::process::Command bug)
/// - **macOS/Linux**: Reads all env vars from shell configuration
pub fn get_all_vars() -> std::result::Result<HashMap<String, String>, Error> {
  get_vars(&[])
}

/// Returns PATH from the login shell without setting it.
///
/// ## Platform-specific
///
/// - **Windows**: Returns current PATH (workaround for std::process::Command bug)
/// - **macOS/Linux**: Reads PATH from shell configuration
pub fn get_path() -> std::result::Result<String, Error> {
  get_vars(&["PATH"])?
    .remove("PATH")
    .ok_or_else(|| Error::InvalidOutput("PATH not found in shell environment".to_string()))
}

/// Reads the shell configuration to properly set all given environment variables.
///
/// ## Platform-specific
///
/// - **Windows**: Refreshes environment variables to fix PATH inheritance bug
///   in std::process::Command where GUI apps don't properly inherit parent PATH
/// - **macOS/Linux**: Reads shell configuration from login shell
pub fn fix_vars(vars: &[&str]) -> std::result::Result<(), Error> {
  for (key, value) in get_vars(vars)? {
    std::env::set_var(key, value);
  }
  Ok(())
}

/// Reads the shell configuration to properly set the PATH environment variable.
///
/// ## Platform-specific
///
/// - **Windows**: Refreshes PATH to fix inheritance bug in std::process::Command
/// - **macOS/Linux**: Reads PATH from shell configuration
pub fn fix() -> std::result::Result<(), Error> {
  fix_vars(&["PATH"])
}

/// Reads the shell configuration to properly set all environment variables.
///
/// ## Platform-specific
///
/// - **Windows**: Refreshes all environment variables to fix inheritance bug
/// - **macOS/Linux**: Reads all environment variables from shell configuration
pub fn fix_all_vars() -> std::result::Result<(), Error> {
  fix_vars(&[])
}

#[cfg(test)]
mod tests {
  use super::*;

  #[cfg(windows)]
  use std::env;

  #[test]
  fn test_fix_returns_ok() {
    // The fix function should always return Ok on all platforms
    assert!(fix().is_ok());
  }

  #[test]
  fn test_fix_vars_returns_ok() {
    // The fix_vars function should always return Ok on all platforms
    assert!(fix_vars(&["PATH"]).is_ok());
  }

  #[test]
  fn test_fix_all_vars_returns_ok() {
    // The fix_all_vars function should always return Ok on all platforms
    assert!(fix_all_vars().is_ok());
  }

  #[cfg(windows)]
  #[test]
  fn test_windows_path_preservation() {
    // On Windows, PATH should exist and be preserved after fix
    let original_path = env::var("PATH").expect("PATH should exist on Windows");
    assert!(fix().is_ok());
    let after_path = env::var("PATH").expect("PATH should still exist after fix");
    assert_eq!(original_path, after_path);
  }

  #[cfg(windows)]
  #[test]
  fn test_windows_custom_var_setting() {
    // Test that custom variables are properly refreshed on Windows
    env::set_var("TEST_VAR_FIX_PATH", "test_value");
    assert!(fix_vars(&["TEST_VAR_FIX_PATH"]).is_ok());
    assert_eq!(env::var("TEST_VAR_FIX_PATH").unwrap(), "test_value");
    env::remove_var("TEST_VAR_FIX_PATH");
  }

  #[cfg(windows)]
  #[test]
  fn test_windows_nonexistent_var() {
    // Test that non-existent variables don't cause errors on Windows
    env::remove_var("NONEXISTENT_VAR_FIX_PATH");
    assert!(fix_vars(&["NONEXISTENT_VAR_FIX_PATH"]).is_ok());
  }

  #[test]
  fn test_get_vars_returns_ok() {
    assert!(get_vars(&["PATH"]).is_ok());
  }

  #[test]
  fn test_get_all_vars_returns_ok() {
    assert!(get_all_vars().is_ok());
  }

  #[test]
  fn test_get_path_returns_ok() {
    let path = get_path();
    assert!(path.is_ok());
    // PATH should not be empty
    assert!(!path.unwrap().is_empty());
  }

  #[test]
  fn test_get_vars_does_not_modify_env() {
    // Set a known value
    std::env::set_var("TEST_GET_VAR", "original");
    // get_vars should not change it
    let _ = get_vars(&["TEST_GET_VAR"]);
    assert_eq!(std::env::var("TEST_GET_VAR").unwrap(), "original");
    std::env::remove_var("TEST_GET_VAR");
  }
}
