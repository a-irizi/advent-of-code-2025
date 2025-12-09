pub mod digits_leading;
pub mod digits_trailing;
pub mod part1;
pub mod part1_leading;
pub mod part2;
pub mod part2_leading;
pub mod part2_parallel;

use std::ops::RangeInclusive;

#[must_use]
pub fn fast_digit_count(number: usize) -> usize {
  match number {
    0..=9 => 1,
    10..=99 => 2,
    100..=999 => 3,
    1000..=9999 => 4,
    10000..=99999 => 5,
    100000..=999999 => 6,
    1000000..=9999999 => 7,
    10000000..=99999999 => 8,
    100000000..=999999999 => 9,
    1000000000..=9999999999 => 10,
    10000000000..=99999999999 => 11,
    100000000000..=999999999999 => 12,
    1000000000000..=9999999999999 => 13,
    10000000000000..=99999999999999 => 14,
    100000000000000..=999999999999999 => 15,
    1000000000000000..=9999999999999999 => 16,
    10000000000000000..=99999999999999999 => 17,
    100000000000000000..=999999999999999999 => 18,
    1000000000000000000..=9999999999999999999 => 19,
    10000000000000000000..=usize::MAX => 20,
    _ => unreachable!("All usize possible ranges are covered"),
  }
}

const POWERS_OF_10: [u64; 20] = [
  1,
  10,
  100,
  1_000,
  10_000,
  100_000,
  1_000_000,
  10_000_000,
  100_000_000,
  1_000_000_000,
  10_000_000_000,
  100_000_000_000,
  1_000_000_000_000,
  10_000_000_000_000,
  100_000_000_000_000,
  1_000_000_000_000_000,
  10_000_000_000_000_000,
  100_000_000_000_000_000,
  1_000_000_000_000_000_000,
  10_000_000_000_000_000_000,
];

const DIVISORS_BY_CHUNK_SIZE: [&[usize]; 21] = [
  &[],               // 0
  &[],               // 1
  &[1],              // 2
  &[1],              // 3
  &[1, 2],           // 4
  &[1],              // 5
  &[1, 2, 3],        // 6
  &[1],              // 7
  &[1, 2, 4],        // 8
  &[1, 3],           // 9
  &[1, 2, 5],        // 10
  &[1],              // 11
  &[1, 2, 3, 4, 6],  // 12
  &[1],              // 13
  &[1, 2, 7],        // 14
  &[1, 3, 5],        // 15
  &[1, 2, 4, 8],     // 16
  &[1],              // 17
  &[1, 2, 3, 6, 9],  // 18
  &[1],              // 19
  &[1, 2, 4, 5, 10], // 20
];

use nom::{
  IResult, Parser, bytes::complete::tag, character::complete, multi::separated_list1,
  sequence::separated_pair,
};

/// parse puzzle input.
///
/// # Returns
/// list of parsed id ranges, and rest of input that failed to
/// conform to input structure.
///
/// # Errors
/// return error if failed to parse input.
pub fn parse_input(input: &str) -> IResult<&str, Vec<RangeInclusive<usize>>> {
  separated_list1(
    tag(","),
    separated_pair(complete::usize, tag("-"), complete::usize).map(|(start, end)| start..=end),
  )
  .parse(input)
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn parse_input_works() {
    let input = "11-22,95-115,998-1012,1188511880-1188511890,\
222220-222224,1698522-1698528,446443-446449,38593856-38593862,\
565653-565659,824824821-824824827,2121212118-2121212124";
    let (rest, ranges) = parse_input(input).unwrap();
    assert_eq!("", rest);
    assert_eq!(
      vec![
        11..=22,
        95..=115,
        998..=1012,
        1_188_511_880..=1_188_511_890,
        222_220..=222_224,
        1_698_522..=1_698_528,
        446_443..=446_449,
        38_593_856..=38_593_862,
        565_653..=565_659,
        824_824_821..=824_824_827,
        2_121_212_118..=2_121_212_124
      ],
      ranges
    );
  }
}
