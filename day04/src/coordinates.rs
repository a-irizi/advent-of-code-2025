#[derive(PartialEq, Eq, Hash, Clone)]
pub struct Coordinate {
  row: u8,
  column: u8,
}

impl Coordinate {
  #[must_use]
  pub fn new(row: u8, column: u8) -> Self {
    Self { row, column }
  }

  #[must_use]
  pub fn top(&self) -> Option<Self> {
    if self.row == 0 {
      return None;
    }

    Some(Self { row: self.row - 1, column: self.column })
  }

  #[must_use]
  pub fn top_right(&self) -> Option<Self> {
    if self.row == 0 {
      return None;
    }

    Some(Self { row: self.row - 1, column: self.column + 1 })
  }

  #[must_use]
  pub fn right(&self) -> Self {
    Self { row: self.row, column: self.column + 1 }
  }

  #[must_use]
  pub fn bottom_right(&self) -> Self {
    Self { row: self.row + 1, column: self.column + 1 }
  }

  #[must_use]
  pub fn bottom(&self) -> Self {
    Self { row: self.row + 1, column: self.column }
  }

  #[must_use]
  pub fn bottom_left(&self) -> Option<Self> {
    if self.column == 0 {
      return None;
    }

    Some(Self { row: self.row + 1, column: self.column - 1 })
  }

  #[must_use]
  pub fn left(&self) -> Option<Self> {
    if self.column == 0 {
      return None;
    }

    Some(Self { row: self.row, column: self.column - 1 })
  }

  #[must_use]
  pub fn top_left(&self) -> Option<Self> {
    if self.row == 0 || self.column == 0 {
      return None;
    }

    Some(Self { row: self.row - 1, column: self.column - 1 })
  }
}
