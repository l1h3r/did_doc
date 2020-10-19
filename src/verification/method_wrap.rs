use core::ops::Deref;

use crate::verification::Method;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MethodWrap<'a> {
  pub(crate) method: &'a Method,
  pub(crate) index: usize,
}

impl<'a> MethodWrap<'a> {
  pub(crate) const fn new(index: usize, method: &'a Method) -> Self {
    Self { index, method }
  }

  pub const fn index(&self) -> usize {
    self.index
  }
}

impl Deref for MethodWrap<'_> {
  type Target = Method;

  fn deref(&self) -> &Self::Target {
    &self.method
  }
}
