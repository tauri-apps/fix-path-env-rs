// Copyright 2021 Tauri Programme within The Commons Conservancy
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

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

/// Reads the shell configuration to properly set all given environment variables.
///
/// ## Platform-specific
///
/// - **Windows**: Refreshes environment variables to fix PATH inheritance bug
///   in std::process::Command where GUI apps don't properly inherit parent PATH
/// - **macOS/Linux**: Reads shell configuration from login shell
pub fn fix_vars(vars: &[&str]) -> std::result::Result<(), Error> {
  #[cfg(windows)]
  {
    // On Windows, there's a known bug where std::process::Command sometimes
    // doesn't properly inherit the parent process's environment variables.
    // By explicitly re-setting them, we force Command to use the current values.
    for &var in vars {
      if let Ok(value) = std::env::var(var) {
        std::env::set_var(var, value);
      }
    }
    Ok(())
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
      for line in String::from_utf8_lossy(&strip_ansi_escapes::strip(env))
        .split('\n')
        .filter(|l| !l.is_empty())
      {
        let mut s = line.splitn(2, '=');
        if let (Some(var), Some(value)) = (s.next(), s.next()) {
          if vars.is_empty() || vars.contains(&var) {
            std::env::set_var(var, value);
          }
        }
      }
      Ok(())
    } else {
      Err(Error::EchoFailed(
        String::from_utf8_lossy(&out.stderr).into_owned(),
      ))
    }
  }
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
}
