pub trait AsDigitsTrailing<T> {
  fn digits_trailing(&self) -> Digits<T>;
}

#[derive(Debug, Clone, Copy)]
pub struct Digits<T> {
  inner: T,
  count: usize,
}

impl<T> Digits<T> {
  pub fn count_fast(&self) -> usize {
    self.count
  }
}

impl Digits<i32> {
  fn new(number: i32) -> Self {
    let count = (number.ilog10() + 1) as usize;
    Self { inner: number, count }
  }
}

impl Digits<usize> {
  fn new(number: usize) -> Self {
    let count = (number.ilog10() + 1) as usize;
    Self { inner: number, count }
  }
}

impl Iterator for Digits<usize> {
  type Item = usize;

  fn next(&mut self) -> Option<Self::Item> {
    if self.inner == 0 {
      return None;
    }

    let next_digit = self.inner.rem_euclid(10);
    self.inner = self.inner.div_euclid(10);

    Some(next_digit)
  }
}

impl Iterator for Digits<i32> {
  type Item = i32;

  fn next(&mut self) -> Option<Self::Item> {
    if self.inner == 0 {
      return None;
    }

    let next_digit = self.inner.rem_euclid(10);
    self.inner = self.inner.div_euclid(10);

    Some(next_digit)
  }
}

impl AsDigitsTrailing<usize> for usize {
  fn digits_trailing(&self) -> Digits<usize> {
    Digits::<usize>::new(*self)
  }
}
