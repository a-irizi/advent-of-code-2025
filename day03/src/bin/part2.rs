use day03::total_joltage_output;
use utils::get_input;

fn main() -> anyhow::Result<()> {
  let input = get_input(env!("CARGO_MANIFEST_DIR"))?;
  let result = run(&input)?;

  println!("Result: {result}");

  Ok(())
}

fn run(input: &str) -> anyhow::Result<usize> {
  total_joltage_output::<12>(input)
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn part2_example_works() {
    let input = "987654321111111
811111111111119
234234234234278
818181911112111";
    let result = run(input).unwrap();

    assert_eq!(result, 3_121_910_778_619);
  }

  #[test]
  fn part2_works() {
    let input = get_input(env!("CARGO_MANIFEST_DIR")).unwrap();
    let result = run(&input).unwrap();

    assert_eq!(result, 169_512_729_575_727);
  }
}
