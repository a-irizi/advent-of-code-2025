use std::fs::read_to_string;

use anyhow::Context;

/// get puzzle input.
///
/// puzzle input file should be named `input.txt` and is expected
/// to be placed at the root of the package.
///
/// # Errors
/// returns error if input file was not found
pub fn get_input(manifest_dir: &str) -> anyhow::Result<String> {
  read_to_string(format!("{manifest_dir}/input.txt"))
    .with_context(|| format!(r#"failed to open "input.txt" at package root {manifest_dir:?}"#))
}
