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
  let mut current_dile = 50;
  for instruction in input.lines() {
    match parse_instruction_complete(instruction)? {
      day01::Instruction::Left(mut ticks) => {
        while ticks > 0 {
          current_dile -= 1;
          ticks -= 1;

          if current_dile == 0 {
            zero_count += 1;
          } else if current_dile < 0 {
            current_dile += 100;
          }
        }
      }
      day01::Instruction::Right(mut ticks) => {
        while ticks > 0 {
          current_dile += 1;
          ticks -= 1;

          if current_dile == 100 {
            zero_count += 1;
            current_dile = 0;
          }
        }
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
