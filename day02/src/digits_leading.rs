use crate::POWERS_OF_10;

pub trait AsDigitsLeading<T> {
  fn digits_leading(&self) -> DigitsLeading<T>;
}

#[derive(Clone, Copy)]
pub struct DigitsLeading<T> {
  inner: T,
  count: usize,
}

impl DigitsLeading<usize> {
  pub fn new(inner: usize) -> Self {
    let count = inner.ilog10() + 1;
    Self { inner, count: count as usize }
  }

  #[inline]
  pub fn fast_count(&self) -> usize {
    self.count
  }
}

impl Iterator for DigitsLeading<usize> {
  type Item = usize;

  fn next(&mut self) -> Option<Self::Item> {
    if self.inner == 0 {
      return None;
    }

    let next_digit = self.inner / POWERS_OF_10[self.count - 1] as usize;
    self.inner = self.inner - next_digit * POWERS_OF_10[self.count - 1] as usize;
    self.count -= 1;

    Some(next_digit)
  }
}

impl AsDigitsLeading<usize> for usize {
  fn digits_leading(&self) -> DigitsLeading<usize> {
    DigitsLeading::new(*self)
  }
}
