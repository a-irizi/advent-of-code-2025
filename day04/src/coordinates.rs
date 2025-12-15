#[derive(PartialEq, Eq, Hash, Clone, Debug)]
pub struct Coordinate {
  row: usize,
  column: usize,
}

impl Coordinate {
  #[must_use]
  pub fn new(row: usize, column: usize) -> Self {
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
  pub fn top_right(&self, width: usize) -> Option<Self> {
    if self.row == 0 || self.column == width - 1 {
      return None;
    }

    Some(Self { row: self.row - 1, column: self.column + 1 })
  }

  #[must_use]
  pub fn right(&self, width: usize) -> Option<Self> {
    if self.column == width - 1 {
      return None;
    }
    Some(Self { row: self.row, column: self.column + 1 })
  }

  #[must_use]
  pub fn bottom_right(&self, width: usize) -> Option<Self> {
    if self.row == width - 1 || self.column == width - 1 {
      return None;
    }

    Some(Self { row: self.row + 1, column: self.column + 1 })
  }

  #[must_use]
  pub fn bottom(&self, width: usize) -> Option<Self> {
    if self.row == width - 1 {
      return None;
    }
    Some(Self { row: self.row + 1, column: self.column })
  }

  #[must_use]
  pub fn bottom_left(&self, width: usize) -> Option<Self> {
    if self.row == width - 1 || self.column == 0 {
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

  #[must_use]
  pub fn row(&self) -> usize {
    self.row
  }

  #[must_use]
  pub fn column(&self) -> usize {
    self.column
  }
}
