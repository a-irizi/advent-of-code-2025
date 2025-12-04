use anyhow::anyhow;
use nom::{
  IResult, Parser, branch::alt, bytes::complete::tag, character::complete::digit1,
  sequence::preceded,
};

pub enum Instruction {
  Left(i32),
  Right(i32),
}

fn distance(input: &str) -> Result<i32, std::num::ParseIntError> {
  input.parse::<i32>()
}

/// parse one instruction line.
///
/// # Returns
/// tuple where first element is the rest of the line. and the second
/// element is the parsed instruction.
///
/// # Errors
/// return error if could not parse the instruction line successfully.
pub fn parse_instruction(input: &str) -> IResult<&str, Instruction> {
  alt((
    preceded(tag("L"), digit1.map_res(distance)).map(Instruction::Left),
    preceded(tag("R"), digit1.map_res(distance)).map(Instruction::Right),
  ))
  .parse(input)
}

/// parse one instruction line.
///
/// # Returns
/// parsed instruction.
///
/// # Errors
/// return error if could not parse the instruction line successfully.
pub fn parse_instruction_complete(input: &str) -> anyhow::Result<Instruction> {
  let Ok((rest, instruction)) = parse_instruction(input) else {
    return Err(anyhow!(format!("Failed to parse instruction: {input:?}")));
  };
  if !rest.is_empty() {
    return Err(anyhow!(format!("Failed to parse instruction: {input:?} -- leftover characters")));
  }

  Ok(instruction)
}
