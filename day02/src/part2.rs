use anyhow::anyhow;

use crate::{DIVISORS_BY_CHUNK_SIZE, POWERS_OF_10, fast_digit_count, parse_input};

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

  let mut total = 0;

  for id_range in id_ranges {
    'id: for id in id_range {
      let digit_count = fast_digit_count(id);
      for &chunk_size in DIVISORS_BY_CHUNK_SIZE[digit_count] {
        let fake_id_chunk = id % POWERS_OF_10[chunk_size] as usize;
        let fake_id = fake_id_chunk
          * ((POWERS_OF_10[digit_count] - 1) / (POWERS_OF_10[chunk_size] - 1)) as usize;
        if fake_id == id {
          total += id;
          continue 'id;
        }
      }
    }
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
