use anyhow::{Context, anyhow};

pub type Joltage = u8;
pub type Bank = Vec<Joltage>;

pub fn total_joltage_output<const N: usize>(input: &str) -> anyhow::Result<usize> {
  input
    .lines()
    .map(|line| {
      let bank = parse_bank(line)?;
      let largest_joltages = max_n_joltage::<N>(&bank)?;
      let joltage_output = joltage_output(largest_joltages);
      Ok(joltage_output)
    })
    .sum()
}

fn parse_bank(input: &str) -> anyhow::Result<Bank> {
  let mut bank: Bank = Bank::new();
  for c in input.chars() {
    match c.to_digit(10) {
      Some(joltage) => {
        bank.push(Joltage::try_from(joltage).context("Failed to cast joltage as u8")?);
      }
      None => return Err(anyhow::anyhow!(format!("{c} is not a valid number"))),
    }
  }

  Ok(bank)
}

fn max_n_joltage<const N: usize>(bank: &[Joltage]) -> anyhow::Result<[Joltage; N]> {
  let indices = find_indices_of_n_largest_joltages::<N>(bank)?;
  let mut chosen_joltages = [0; N];
  for (i, &idx) in indices.iter().enumerate() {
    chosen_joltages[i] = bank[idx];
  }

  Ok(chosen_joltages)
}

fn find_indices_of_n_largest_joltages<const N: usize>(
  bank: &[Joltage],
) -> anyhow::Result<[usize; N]> {
  if bank.len() < N {
    return Err(anyhow::anyhow!(format!(
      "Required max {N} joltages in a bank of {battery_count} batteries",
      battery_count = bank.len()
    )));
  }

  let mut selected_indices = [0; N];
  let mut search_start = 0;
  let mut batteries_left_to_select = N;

  for select_index in 0..N {
    let unchecked_batteries_count = bank.len() - search_start;
    // select all remaining batteries if their number is the same as
    // the number of batteries we still need.
    if unchecked_batteries_count == batteries_left_to_select {
      for i in select_index..N {
        selected_indices[i] = search_start + (i - select_index);
      }
      break;
    }

    // normal search for maximum in current window

    let last_valid_start = bank.len() - (batteries_left_to_select - 1);
    let searchable_window = &bank[search_start..last_valid_start];

    let max_index = find_first_maximum_index(searchable_window)
      .map(|relative_index| relative_index + search_start)
      .ok_or(anyhow!(
        "Failed to get max joltage on bank window {:?} of bank {:?}",
        &bank[search_start..last_valid_start],
        bank
      ))?;

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
