use anyhow::{Context, anyhow};

pub type Joltage = u8;
pub type Bank<const N: usize> = [Joltage; N];

pub fn run<const CAPACITY: usize>(input: &str) -> anyhow::Result<usize> {
  total_joltage_output::<12, CAPACITY>(input)
}

pub fn total_joltage_output<const N: usize, const CAPACITY: usize>(
  input: &str,
) -> anyhow::Result<usize> {
  input
    .lines()
    .map(|line| {
      let bank = parse_bank::<CAPACITY>(line)?;
      let largest_joltages = max_n_joltage::<N, CAPACITY>(bank)?;
      let joltage_output = joltage_output(largest_joltages);
      Ok(joltage_output)
    })
    .sum()
}

fn parse_bank<const CAPACITY: usize>(input: &str) -> anyhow::Result<Bank<CAPACITY>> {
  let input = input.as_bytes();
  if input.len() != CAPACITY {
    return Err(anyhow!("incorrect bank capacity, provided {CAPACITY} but found {}", input.len()));
  }

  let mut bank = [0; CAPACITY];
  for i in 0..CAPACITY {
    bank[i] = input[i].wrapping_sub(b'0');
    if bank[i] > 9 {
      return Err(anyhow!("Invalid digit {}", bank[i]));
    }
  }

  Ok(bank)
}

fn max_n_joltage<const N: usize, const CAPACITY: usize>(
  bank: Bank<CAPACITY>,
) -> anyhow::Result<[Joltage; N]> {
  let indices = find_indices_of_n_largest_joltages::<N, CAPACITY>(bank)?;
  let mut chosen_joltages = [0; N];
  for (i, &idx) in indices.iter().enumerate() {
    chosen_joltages[i] = bank[idx];
  }

  Ok(chosen_joltages)
}

fn find_indices_of_n_largest_joltages<const N: usize, const CAPACITY: usize>(
  bank: Bank<CAPACITY>,
) -> anyhow::Result<[usize; N]> {
  let mut selected_indices = [0; N];
  let mut search_start = 0;
  let mut batteries_left_to_select = N;

  for select_index in 0..N {
    let unchecked_batteries_count = CAPACITY - search_start;
    // select all remaining batteries if their number is the same as
    // the number of batteries we still need.
    if unchecked_batteries_count == batteries_left_to_select {
      for i in select_index..N {
        selected_indices[i] = search_start + (i - select_index);
      }
      break;
    }

    // normal search for maximum in current window

    let last_valid_start = CAPACITY - (batteries_left_to_select - 1);
    let searchable_window = &bank[search_start..last_valid_start];

    let max_index = find_first_maximum_index(searchable_window)
      .map(|relative_index| relative_index + search_start)
      .ok_or_else(|| {
        anyhow!(
          "Failed to get max joltage on bank window {:?} of bank {:?}",
          &bank[search_start..last_valid_start],
          bank
        )
      })?;

    search_start = max_index + 1;
    batteries_left_to_select -= 1;
    selected_indices[select_index] = max_index;
  }

  Ok(selected_indices)
}

fn joltage_output<const N: usize>(joltages: [Joltage; N]) -> usize {
  joltages.into_iter().fold(0usize, |acc, current| acc * 10 + usize::from(current))
}

/// Finds the first occurrence of the maximum digit in a slice and returns its index and value.
///
/// # Returns
/// - `Some(index)` where `index` is the position of the *first* maximum digit.
/// - `None` if the input slice is empty.
fn find_first_maximum_index(bank: &[Joltage]) -> Option<usize> {
  bank
    .iter()
    .copied()
    .enumerate()
    .reduce(|first_max, current| if first_max.1 < current.1 { current } else { first_max })
    .map(|max| max.0)
}

#[cfg(test)]
mod tests {
  use utils::get_input;

  use super::*;

  #[test]
  fn part2_example_works() {
    let input = "987654321111111
811111111111119
234234234234278
818181911112111";
    let result = run::<15>(input).unwrap();

    assert_eq!(result, 3_121_910_778_619);
  }

  #[test]
  fn part2_works() {
    let input = get_input(env!("CARGO_MANIFEST_DIR")).unwrap();
    let result = run::<100>(&input).unwrap();

    assert_eq!(result, 169_512_729_575_727);
  }
}
