pub mod coordinates;
pub mod part1;
pub mod part2;

fn is_paper_roll(input: &str) -> bool {
  matches!(input, "@")
}

fn is_paper_roll_char(input: char) -> bool {
  matches!(input, '@')
}
