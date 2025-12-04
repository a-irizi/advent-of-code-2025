mod part1;
mod part2;

use std::fs::read_to_string;

use anyhow::Context;

fn main() -> anyhow::Result<()> {
  let input =
    read_to_string("input.txt").context(r#"failed to open "input.txt" at package root"#)?;
  let part1_result = part1::run(&input)?;
  let part2_result = part2::run(&input)?;
  println!("part1 result {part1_result}");
  println!("part2 result {part2_result}");

  Ok(())
}
