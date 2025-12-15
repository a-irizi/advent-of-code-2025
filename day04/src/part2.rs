use std::collections::HashSet;

use crate::is_paper_roll;

pub fn run(input: &str) -> usize {
  let mut removed_rolls: HashSet<(usize, usize)> = HashSet::new();
  let mut total_count = 0;
  loop {
    let accessible_rolls = get_accessible_rolls(input, &removed_rolls);
    let no_accessible_rolls_left = accessible_rolls.is_empty();
    if no_accessible_rolls_left {
      break;
    }
    total_count += accessible_rolls.len();
    for roll_coordinates in accessible_rolls {
      removed_rolls.insert(roll_coordinates);
    }
  }

  total_count
}

fn get_accessible_rolls<S: ::std::hash::BuildHasher>(
  input: &str,
  removed_rolls: &HashSet<(usize, usize), S>,
) -> Vec<(usize, usize)> {
  let mut accessible_rolls = vec![];
  let mut prev_row: Option<&str> = None;
  let mut rows = input.lines().enumerate().peekable();

  while let Some((row_index, row)) = rows.next() {
    let next_row = rows.peek().map(|row| row.1);
    for column_index in 0..row.len() {
      let cell_is_not_a_roll = !row.get(column_index..column_index + 1).is_some_and(is_paper_roll);
      let roll_already_removed = removed_rolls.contains(&(row_index, column_index));
      if cell_is_not_a_roll || roll_already_removed {
        continue;
      }

      let count =
        count_adjacent_paper_rolls(prev_row, row, next_row, row_index, column_index, removed_rolls);

      if count < 4 {
        accessible_rolls.push((row_index, column_index));
      }
    }
    prev_row = Some(row);
  }

  accessible_rolls
}

pub fn count_adjacent_paper_rolls<S: ::std::hash::BuildHasher>(
  prev_row: Option<&str>,
  row: &str,
  next_row: Option<&str>,
  row_index: usize,
  column_index: usize,
  removed_rolls: &HashSet<(usize, usize), S>,
) -> usize {
  let prev_count = prev_row
    .map(|row| {
      count_3_existing_paper_rolls(row, row_index.saturating_sub(1), column_index, removed_rolls)
    })
    .unwrap_or_default();
  let side_count =
    count_3_existing_paper_rolls(row, row_index, column_index, removed_rolls).saturating_sub(1);
  let next_count = next_row
    .map(|row| count_3_existing_paper_rolls(row, row_index + 1, column_index, removed_rolls))
    .unwrap_or_default();

  prev_count + side_count + next_count
}

fn count_3_existing_paper_rolls<S: ::std::hash::BuildHasher>(
  row: &str,
  row_index: usize,
  column_index: usize,
  removed_rolls: &HashSet<(usize, usize), S>,
) -> usize {
  let mut count = 0;

  if column_index > 0
    && !removed_rolls.contains(&(row_index, column_index - 1))
    && row.get(column_index - 1..column_index).is_some_and(is_paper_roll)
  {
    count += 1;
  }

  if !removed_rolls.contains(&(row_index, column_index))
    && row.get(column_index..column_index + 1).is_some_and(is_paper_roll)
  {
    count += 1;
  }

  if !removed_rolls.contains(&(row_index, column_index + 1))
    && row.get(column_index + 1..column_index + 2).is_some_and(is_paper_roll)
  {
    count += 1;
  }

  count
}

#[cfg(test)]
mod tests {
  use utils::get_input;

  use super::*;

  #[test]
  fn part2_example_works() {
    let input = "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.";
    let result = run(input);
    assert_eq!(result, 43);
  }

  #[test]
  fn part2_works() {
    let input = get_input(env!("CARGO_MANIFEST_DIR")).unwrap();
    let result = run(&input);
    assert_eq!(result, 8746);
  }
}
