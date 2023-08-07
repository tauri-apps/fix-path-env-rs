#[cfg(not(windows))]
mod tests {
  use fix_path_env::*;

  #[test]
  fn test_fix_vars() {
    // Test setting a single environment variable.
    std::env::set_var("TEST_VAR", "test_value");
    fix_vars(&["TEST_VAR"]).unwrap();
    assert_eq!(std::env::var("TEST_VAR"), Ok(String::from("test_value")));

    // Test setting multiple environment variables.
    std::env::set_var("TEST_VAR_1", "test_value_1");
    std::env::set_var("TEST_VAR_2", "test_value_2");
    fix_vars(&["TEST_VAR_1", "TEST_VAR_2"]).unwrap();
    assert_eq!(std::env::var("TEST_VAR_1"), Ok(String::from("test_value_1")));
    assert_eq!(std::env::var("TEST_VAR_2"), Ok(String::from("test_value_2")));

    // Test setting all environment variables.
    std::env::set_var("TEST_VAR_1", "test_value_1");
    std::env::set_var("TEST_VAR_2", "test_value_2");
    std::env::set_var("TEST_VAR_3", "test_value_3");
    fix_all_vars().unwrap();
    assert_eq!(std::env::var("TEST_VAR_1"), Ok(String::from("test_value_1")));
    assert_eq!(std::env::var("TEST_VAR_2"), Ok(String::from("test_value_2")));
    assert_eq!(std::env::var("TEST_VAR_3"), Ok(String::from("test_value_3")));
  }

  #[test]
  fn test_fix_vars_newlines_in_vars() {
    std::env::set_var("TEST_VAR", "test_value\nwith\nnewlines");
    fix_vars(&["TEST_VAR"]).unwrap();
    assert_eq!(
      std::env::var("TEST_VAR"),
      Ok(String::from("test_value\nwith\nnewlines"))
    );
  }
}