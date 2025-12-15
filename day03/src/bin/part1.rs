use day03::part1;
use utils::get_input;

fn main() -> anyhow::Result<()> {
  let input = get_input(env!("CARGO_MANIFEST_DIR"))?;
  let result = part1::run(&input)?;

  println!("Result: {result}");

  Ok(())
}
