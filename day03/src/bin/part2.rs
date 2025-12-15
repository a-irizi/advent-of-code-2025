use day03::{part2, part2_const_bank_size};
use utils::get_input;

fn main() -> anyhow::Result<()> {
  let input = get_input(env!("CARGO_MANIFEST_DIR"))?;
  let result = part2_const_bank_size::run::<100>(&input)?;

  println!("Result: {result}");

  Ok(())
}
