use anyhow::Context;

pub fn run(input: &str) -> anyhow::Result<i32> {
  let mut zero_count = 0;
  let mut current_dile = 50;
  for instruction in input.lines() {
    let direction = &instruction[..1];
    let mut distance_left: i32 = instruction[1..].parse().context("distance should be a number")?;
    if direction == "L" {
      while distance_left > 0 {
        current_dile -= 1;
        distance_left -= 1;

        if current_dile == 0 {
          zero_count += 1;
        } else if current_dile < 0 {
          current_dile += 100;
        }
      }
    } else {
      while distance_left > 0 {
        current_dile += 1;
        distance_left -= 1;

        if current_dile == 100 {
          zero_count += 1;
          current_dile = 0;
        }
      }
    }
  }

  Ok(zero_count)
}

#[cfg(test)]
mod tests {
  use std::fs::read_to_string;

  use super::*;

  #[test]
  fn part2_example_works() {
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
    assert_eq!(result, 6);
  }

  #[test]
  fn part2_works() {
    let input = read_to_string("input.txt").unwrap();
    let result = run(&input).unwrap();
    assert_eq!(result, 6689);
  }
}
