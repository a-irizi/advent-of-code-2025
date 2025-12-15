use anyhow::anyhow;
use bitvector::BitVector;

use crate::{coordinates::Coordinate, is_paper_roll_char};

pub fn run(input: &str) -> anyhow::Result<usize> {
  let grid_width = input.lines().next().ok_or_else(|| anyhow!("Got empty input"))?.len();
  let mut paper_rolls = get_paper_rolls(grid_width, input);
  let mut total_accessed_paper_rolls = 0;
  loop {
    let accessed_paper_rolls = remove_accessible_paper_rolls(grid_width, &mut paper_rolls);
    if accessed_paper_rolls == 0 {
      break;
    }
    total_accessed_paper_rolls += accessed_paper_rolls;
  }

  Ok(total_accessed_paper_rolls)
}

fn get_paper_rolls(grid_width: usize, input: &str) -> BitVector {
  let grid_size = grid_width * grid_width;
  let mut coordinates = BitVector::new(grid_size);
  for (row_index, row) in input.lines().enumerate() {
    for (column_index, position) in row.chars().enumerate() {
      if is_paper_roll_char(position) {
        coordinates.insert(get_linear_coordinate(row.len(), row_index, column_index));
      }
    }
  }

  coordinates
}

fn get_linear_coordinate(width: usize, row: usize, column: usize) -> usize {
  row * width + column
}

fn remove_accessible_paper_rolls(grid_width: usize, paper_rolls: &mut BitVector) -> usize {
  let accessible_paper_rolls = get_accessible_paper_rolls(grid_width, paper_rolls);
  let accessible_paper_rolls_count = accessible_paper_rolls.len();
  for accessible_paper_roll in accessible_paper_rolls {
    paper_rolls.remove(accessible_paper_roll);
  }

  accessible_paper_rolls_count
}

fn get_accessible_paper_rolls(grid_width: usize, paper_rolls: &BitVector) -> Vec<usize> {
  let mut accessible_paper_rolls = vec![];

  for paper_roll in paper_rolls {
    if is_paper_roll_accessible(grid_width, paper_roll, paper_rolls) {
      accessible_paper_rolls.push(paper_roll);
    }
  }

  accessible_paper_rolls
}

fn is_paper_roll_accessible(grid_width: usize, paper_roll: usize, paper_rolls: &BitVector) -> bool {
  let mut adjacent_paper_rolls_count = 0;

  let row = paper_roll / grid_width;
  let column = paper_roll % grid_width;
  let coordinates = Coordinate::new(row, column);

  if let Some(adjacent_roll) = coordinates.top()
    && paper_rolls.contains(get_linear_coordinate(
      grid_width,
      adjacent_roll.row(),
      adjacent_roll.column(),
    ))
  {
    adjacent_paper_rolls_count += 1;
  }

  if let Some(adjacent_roll) = coordinates.top_right(grid_width)
    && paper_rolls.contains(get_linear_coordinate(
      grid_width,
      adjacent_roll.row(),
      adjacent_roll.column(),
    ))
  {
    adjacent_paper_rolls_count += 1;
  }

  if let Some(adjacent_roll) = coordinates.right(grid_width)
    && paper_rolls.contains(get_linear_coordinate(
      grid_width,
      adjacent_roll.row(),
      adjacent_roll.column(),
    ))
  {
    adjacent_paper_rolls_count += 1;
  }

  if let Some(adjacent_roll) = coordinates.bottom_right(grid_width)
    && paper_rolls.contains(get_linear_coordinate(
      grid_width,
      adjacent_roll.row(),
      adjacent_roll.column(),
    ))
  {
    adjacent_paper_rolls_count += 1;
  }

  if let Some(adjacent_roll) = coordinates.bottom(grid_width)
    && paper_rolls.contains(get_linear_coordinate(
      grid_width,
      adjacent_roll.row(),
      adjacent_roll.column(),
    ))
  {
    adjacent_paper_rolls_count += 1;
  }

  if let Some(adjacent_roll) = coordinates.bottom_left(grid_width)
    && paper_rolls.contains(get_linear_coordinate(
      grid_width,
      adjacent_roll.row(),
      adjacent_roll.column(),
    ))
  {
    adjacent_paper_rolls_count += 1;
  }

  if let Some(adjacent_roll) = coordinates.left()
    && paper_rolls.contains(get_linear_coordinate(
      grid_width,
      adjacent_roll.row(),
      adjacent_roll.column(),
    ))
  {
    adjacent_paper_rolls_count += 1;
  }

  if let Some(top_left) = coordinates.top_left()
    && paper_rolls.contains(get_linear_coordinate(grid_width, top_left.row(), top_left.column()))
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
    let result = run(input).unwrap();
    assert_eq!(result, 43);
  }

  #[test]
  fn part2_works() {
    let input = get_input(env!("CARGO_MANIFEST_DIR")).unwrap();
    let result = run(&input).unwrap();
    assert_eq!(result, 8746);
  }
}
