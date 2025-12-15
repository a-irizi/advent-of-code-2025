use crate::total_joltage_output;

pub fn run(input: &str) -> anyhow::Result<usize> {
  total_joltage_output::<2>(input)
}

#[cfg(test)]
mod tests {
  use utils::get_input;

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
