use crate::is_paper_roll;

pub fn run(input: &str) -> usize {
  let mut prev_row: Option<&str> = None;
  let mut total_count = 0;
  let mut rows = input.lines().peekable();
  while let Some(row) = rows.next() {
    let next_row = rows.peek().copied();
    for column_index in 0..row.len() {
      if !row.get(column_index..column_index + 1).is_some_and(is_paper_roll) {
        continue;
      }
      let count = count_adjacent_paper_rolls(prev_row, row, next_row, column_index);
      if count < 4 {
        total_count += 1;
      }
    }
    prev_row = Some(row);
  }

  total_count
}

pub fn count_adjacent_paper_rolls(
  prev_row: Option<&str>,
  row: &str,
  next_row: Option<&str>,
  column: usize,
) -> usize {
  let prev_count =
    prev_row.map(|row| count_3_existing_paper_rolls(row, column)).unwrap_or_default();
  let side_count = count_3_existing_paper_rolls(row, column).saturating_sub(1);
  let next_count =
    next_row.map(|row| count_3_existing_paper_rolls(row, column)).unwrap_or_default();

  prev_count + side_count + next_count
}

fn count_3_existing_paper_rolls(row: &str, column: usize) -> usize {
  let mut count = 0;

  if column > 0 && row.get(column - 1..column).is_some_and(is_paper_roll) {
    count += 1;
  }

  if row.get(column..column + 1).is_some_and(is_paper_roll) {
    count += 1;
  }

  if row.get(column + 1..column + 2).is_some_and(is_paper_roll) {
    count += 1;
  }

  count
}

#[cfg(test)]
mod tests {
  use utils::get_input;

  use super::*;

  #[test]
  fn part1_example_works() {
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
    assert_eq!(result, 13);
  }

  #[test]
  fn part1_works() {
    let input = get_input(env!("CARGO_MANIFEST_DIR")).unwrap();
    let result = run(&input);
    assert_eq!(result, 1449);
  }
}
