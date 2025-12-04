use anyhow::Context;
use utils::get_input;

fn main() -> anyhow::Result<()> {
  let input = get_input(env!("CARGO_MANIFEST_DIR"))?;
  let result = run(&input)?;

  println!("Result: {result}");

  Ok(())
}

fn run(input: &str) -> anyhow::Result<i32> {
  let mut zero_count = 0;
  let mut current_dile = 50;
  for instruction in input.lines() {
    let distance: i32 =
      instruction[1..].parse().context("distance after direction should be valid number")?;

    current_dile = if &instruction[..1] == "L" {
      (current_dile - distance) % 100
    } else {
      (current_dile + distance) % 100
    };

    if current_dile == 0 {
      zero_count += 1;
    }
  }

  Ok(zero_count)
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn part1_example_works() {
    let input = "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82";
    let result = run(input).unwrap();
    assert_eq!(result, 3);
  }

  #[test]
  fn part1_works() {
    let input = get_input(env!("CARGO_MANIFEST_DIR")).unwrap();
    let result = run(&input).unwrap();
    assert_eq!(result, 1081);
  }
}
