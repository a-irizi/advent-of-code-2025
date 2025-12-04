use day01::parse_instruction_complete;
use utils::get_input;

fn main() -> anyhow::Result<()> {
  let input = get_input(env!("CARGO_MANIFEST_DIR"))?;
  let result = run(&input)?;

  println!("Result: {result}");

  Ok(())
}

fn run(input: &str) -> anyhow::Result<i32> {
  let mut zero_count = 0;
  let mut current_dial = 50;
  for instruction in input.lines() {
    match parse_instruction_complete(instruction)? {
      day01::Instruction::Left(ticks) => {
        let ticks_until_zero = if current_dial == 0 { 100 } else { current_dial };
        if ticks >= ticks_until_zero {
          zero_count += 1;
          zero_count += (ticks - ticks_until_zero) / 100;
        }
        current_dial = (current_dial - ticks).rem_euclid(100);
      }
      day01::Instruction::Right(ticks) => {
        let ticks_until_zero = if current_dial == 0 { 100 } else { 100 - current_dial };
        if ticks >= ticks_until_zero {
          zero_count += 1;
          zero_count += (ticks - ticks_until_zero) / 100;
        }
        current_dial = (current_dial + ticks).rem_euclid(100);
      }
    }
  }

  Ok(zero_count)
}

#[cfg(test)]
mod tests {
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
    let input = get_input(env!("CARGO_MANIFEST_DIR")).unwrap();
    let result = run(&input).unwrap();
    assert_eq!(result, 6689);
  }
}
