use std::sync::{
  atomic::{AtomicUsize, Ordering},
  mpsc,
};

use anyhow::anyhow;
use rayon::prelude::*;

use crate::{digits_trailing::AsDigitsTrailing, parse_input};

/// solve day 02 part 2 puzzle.
///
/// # Returns
/// sum of invalid ids.
///
/// # Errors
/// returns error if failed to parse input.
pub fn run(input: &str) -> anyhow::Result<usize> {
  let Ok((rest, id_ranges)) = parse_input(input) else {
    return Err(anyhow!("Failed to parse input: {input:?}"));
  };

  if !rest.is_empty() {
    return Err(anyhow!("Failed to parse input: {input:?} -- left over characters {rest:?}"));
  }

  let (tx, rx) = mpsc::channel::<usize>();

  id_ranges.into_par_iter().for_each_with(tx, |tx, id_range| {
    'id: for id in id_range {
      let id_digits = id.digits_trailing();
      let digit_count = id_digits.count_fast();
      for chunk_size in 1..=digit_count / 2 {
        if !digit_count.is_multiple_of(chunk_size) {
          continue;
        }
        let chunk_count = digit_count / chunk_size;
        let fake_id_chunk =
          id_digits.take(chunk_size).enumerate().fold(0, |acc, (idx, current)| {
            acc + current * 10usize.pow(idx.try_into().expect("Got very big exponent"))
          });
        let mut fake_id = fake_id_chunk;
        for exponent in 1..chunk_count {
          fake_id += fake_id_chunk * 10usize.pow((chunk_size * exponent) as u32);
        }
        if fake_id == id {
          tx.send(id).unwrap();
          continue 'id;
        }
      }
    }
  });

  let mut total = 0;
  while let Ok(fake_id) = rx.recv() {
    total += fake_id;
  }

  Ok(total)
}

#[cfg(test)]
mod tests {
  use utils::get_input;

  use super::*;

  #[test]
  fn part2_example_works() {
    let input = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,\
1698522-1698528,446443-446449,38593856-38593862,565653-565659,\
824824821-824824827,2121212118-2121212124";
    let result = run(input).unwrap();
    assert_eq!(result, 4_174_379_265);
  }

  #[test]
  fn part2_works() {
    let input = get_input(env!("CARGO_MANIFEST_DIR")).unwrap();
    let result = run(&input).unwrap();
    assert_eq!(result, 69_553_832_684);
  }
}
