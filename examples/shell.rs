// Copyright 2021 Tauri Programme within The Commons Conservancy
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

use fix_path_env::fix;

fn main() {
  if let Err(e) = fix() {
    println!("{}", e);
  } else {
    println!("PATH: {}", std::env::var("PATH").unwrap());
  }
}
