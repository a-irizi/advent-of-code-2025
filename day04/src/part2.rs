use std::collections::HashSet;

use crate::{coordinates::Coordinate, is_paper_roll_char};

pub fn run(input: &str) -> usize {
  let mut paper_rolls = get_paper_rolls(input);
  let mut total_accessed_paper_rolls = 0;
  loop {
    let accessed_paper_rolls = remove_accessible_paper_rolls(&mut paper_rolls);
    if accessed_paper_rolls == 0 {
      break;
    }
    total_accessed_paper_rolls += accessed_paper_rolls;
  }

  total_accessed_paper_rolls
}

fn get_paper_rolls(input: &str) -> HashSet<Coordinate> {
  let mut coordinates = HashSet::new();
  for (row_index, row) in input.lines().enumerate() {
    for (column_index, position) in row.chars().enumerate() {
      if is_paper_roll_char(position) {
        coordinates.insert(Coordinate::new(row_index as u8, column_index as u8));
      }
    }
  }

  coordinates
}

fn remove_accessible_paper_rolls(paper_rolls: &mut HashSet<Coordinate>) -> usize {
  let accessible_paper_rolls = get_accessible_paper_rolls(paper_rolls);
  for accessible_paper_roll in &accessible_paper_rolls {
    paper_rolls.remove(accessible_paper_roll);
  }

  accessible_paper_rolls.len()
}

fn get_accessible_paper_rolls(paper_rolls: &HashSet<Coordinate>) -> Vec<Coordinate> {
  let mut accessible_paper_rolls = vec![];

  for paper_roll in paper_rolls {
    if is_paper_roll_accessible(paper_roll, paper_rolls) {
      accessible_paper_rolls.push(paper_roll.clone());
    }
  }

  accessible_paper_rolls
}

fn is_paper_roll_accessible(paper_roll: &Coordinate, paper_rolls: &HashSet<Coordinate>) -> bool {
  let mut adjacent_paper_rolls_count = 0;

  if let Some(top) = paper_roll.top()
    && paper_rolls.contains(&top)
  {
    adjacent_paper_rolls_count += 1;
  }

  if let Some(top_right) = paper_roll.top_right()
    && paper_rolls.contains(&top_right)
  {
    adjacent_paper_rolls_count += 1;
  }

  if paper_rolls.contains(&paper_roll.right()) {
    adjacent_paper_rolls_count += 1;
  }

  if paper_rolls.contains(&paper_roll.bottom_right()) {
    adjacent_paper_rolls_count += 1;
  }

  if paper_rolls.contains(&paper_roll.bottom()) {
    adjacent_paper_rolls_count += 1;
  }

  if let Some(bottom_left) = paper_roll.bottom_left()
    && paper_rolls.contains(&bottom_left)
  {
    adjacent_paper_rolls_count += 1;
  }

  if let Some(left) = paper_roll.left()
    && paper_rolls.contains(&left)
  {
    adjacent_paper_rolls_count += 1;
  }

  if let Some(top_left) = paper_roll.top_left()
    && paper_rolls.contains(&top_left)
  {
    adjacent_paper_rolls_count += 1;
  }

  adjacent_paper_rolls_count < 4
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
