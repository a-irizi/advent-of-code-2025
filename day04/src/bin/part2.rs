use day04::part2::run;
use utils::get_input;

fn main() -> anyhow::Result<()> {
  let input = get_input(env!("CARGO_MANIFEST_DIR"))?;
  let result = run(&input)?;

  println!("Result: {result}");

  Ok(())
}
