pub mod digits_leading;
pub mod digits_trailing;
pub mod part1;
pub mod part1_leading;
pub mod part2;
pub mod part2_leading;
pub mod part2_parallel;

use std::ops::RangeInclusive;

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
