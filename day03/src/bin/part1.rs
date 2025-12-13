use anyhow::anyhow;
use day03::{get_biggest_joltage, parse_bank};
use utils::get_input;

fn main() -> anyhow::Result<()> {
  let input = get_input(env!("CARGO_MANIFEST_DIR"))?;
  let result = run(&input)?;

  println!("Result: {result}");

  Ok(())
}

fn run(input: &str) -> anyhow::Result<usize> {
  let mut total_joltage: usize = 0;
  let battery_count = 2;
  for bank in input.lines() {
    let bank = parse_bank(bank)?;
    let (idx, first_max_joltage) =
      get_biggest_joltage(&bank[..bank.len() - (battery_count - 1)])
        .ok_or_else(|| anyhow!("Failed to get first max joltage for bank {bank:?}"))?;
    let (_, second_max_joltage) =
      get_biggest_joltage(&bank[idx + 1..bank.len() - (battery_count - 2)])
        .ok_or_else(|| anyhow!("Failed to get second max joltage for bank {bank:?}"))?;
    let joltage = first_max_joltage * 10 + second_max_joltage;

    total_joltage += usize::from(joltage);
  }

  Ok(total_joltage)
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn part1_example_works() {
    let input = "987654321111111
811111111111119
234234234234278
818181911112111";
    let result = run(input).unwrap();

    assert_eq!(result, 357);
  }

  #[test]
  fn part1_works() {
    let input = get_input(env!("CARGO_MANIFEST_DIR")).unwrap();
    let result = run(&input).unwrap();

    assert_eq!(result, 17_074);
  }
}
