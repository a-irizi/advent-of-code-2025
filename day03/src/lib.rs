use anyhow::Context;

pub type Joltage = u8;
pub type Bank = Vec<Joltage>;

pub fn parse_bank(input: &str) -> anyhow::Result<Bank> {
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

pub fn get_biggest_joltage(bank: &[Joltage]) -> Option<(usize, Joltage)> {
  bank
    .iter()
    .enumerate()
    .reduce(
      |(max_idx, max_joltage), (idx, joltage)| {
        if joltage > max_joltage { (idx, joltage) } else { (max_idx, max_joltage) }
      },
    )
    .map(|(idx, joltage)| (idx, *joltage))
}


