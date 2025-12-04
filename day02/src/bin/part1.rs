use anyhow::anyhow;
use day02::AsDigits;
use day02::parse_input;
use itertools::Itertools;
use utils::get_input;

fn main() -> anyhow::Result<()> {
  let input = get_input(env!("CARGO_MANIFEST_DIR"))?;
  let result = run(&input)?;

  println!("Result: {result}");

  Ok(())
}

fn run(input: &str) -> anyhow::Result<usize> {
  let Ok((rest, id_ranges)) = parse_input(input) else {
    return Err(anyhow!("Failed to parse input: {input:?}"));
  };

  if !rest.is_empty() {
    return Err(anyhow!("Failed to parse input: {input:?} -- left over characters {rest:?}"));
  }

  let mut fake_ids = vec![];

  for id_range in id_ranges {
    for id in id_range {
      let id_digits = id.digits().collect_vec();
      if !id_digits.len().is_multiple_of(2) {
        continue;
      }
      let mut id_chunks = id_digits.chunks_exact(id_digits.len() / 2);
      let is_fake_id = id_chunks.all_equal();
      if is_fake_id && id_chunks.remainder().is_empty() {
        fake_ids.push(id);
      }
    }
  }

  Ok(fake_ids.iter().sum())
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn part1_example_works() {
    let input = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,\
1698522-1698528,446443-446449,38593856-38593862,565653-565659,\
824824821-824824827,2121212118-2121212124";
    let result = run(input).unwrap();
    assert_eq!(result, 1_227_775_554);
  }

  #[test]
  fn part1_works() {
    let input = get_input(env!("CARGO_MANIFEST_DIR")).unwrap();
    let result = run(&input).unwrap();
    assert_eq!(result, 53_420_042_388);
  }
}
