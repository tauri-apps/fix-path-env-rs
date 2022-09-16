// Copyright 2021 Tauri Programme within The Commons Conservancy
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

/// The error that might happen on a [`fix`] call.
#[derive(Debug, thiserror::Error)]
pub enum Error {
  #[error(transparent)]
  Shell(#[from] std::io::Error),
  #[error("failed to run shell echo: {0}")]
  EchoFailed(String),
}


/// Reads the shell configuration to properly set all given environment variables.
#[cfg(not(windows))]
pub fn fix_vars(vars: &[&str]) -> std::result::Result<(), Error> {
  let default_shell = if cfg!(target_os = "macos") {
    "/bin/zsh"
  } else {
    "/bin/sh"
  };
  let shell = std::env::var("SHELL").unwrap_or_else(|_| default_shell.into());

  let out = std::process::Command::new(shell)
    .arg("-ilc")
    .arg("echo -n \"_SHELL_ENV_DELIMITER_\"; env; echo -n \"_SHELL_ENV_DELIMITER_\"; exit")
    // Disables Oh My Zsh auto-update thing that can block the process.
    .env("DISABLE_AUTO_UPDATE", "true")
    .output()
    .map_err(Error::Shell)?;

  if out.status.success() {
    let stdout = String::from_utf8_lossy(&out.stdout).into_owned();
    let env = stdout.split("_SHELL_ENV_DELIMITER_").nth(1).unwrap();
    for line in String::from_utf8_lossy(&strip_ansi_escapes::strip(env)?)
      .split('\n')
      .filter(|l| !l.is_empty())
    {
      let mut s = line.split('=');
      if let (Some(var), Some(value)) = (s.next(), s.next()) {
        if vars.contains(&var) {
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

/// Reads the shell configuration to properly set the PATH environment variable.
#[cfg(not(windows))]
pub fn fix() -> std::result::Result<(), Error> {
  fix_vars(&["PATH"])
}

/// Do nothing on Windows as the PATH environment variable is already set.
#[cfg(windows)]
pub fn fix() -> std::result::Result<(), Error> {
  Ok(())
}
