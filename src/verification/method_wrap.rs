use alloc::string::String;
use core::iter::once;
use core::ops::Deref;

use crate::error::Error;
use crate::error::Result;
use crate::utils::Object;
use crate::verification::Method;
use crate::verification::MethodScope;

const ERR_VMMF: &str = "Verification Method Missing Fragment";

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MethodWrap<'a, T = Object> {
  pub(crate) method: &'a Method<T>,
  pub(crate) index: usize,
  pub(crate) scope: MethodScope,
}

impl<'a, T> MethodWrap<'a, T> {
  pub(crate) const fn new(method: &'a Method<T>, index: usize, scope: MethodScope) -> Self {
    Self {
      index,
      method,
      scope,
    }
  }

  pub const fn index(&self) -> usize {
    self.index
  }

  pub const fn scope(&self) -> MethodScope {
    self.scope
  }

  pub const fn into_method(self) -> &'a Method<T> {
    self.method
  }

  pub fn try_into_fragment(&self) -> Result<String> {
    self
      .id()
      .fragment()
      .ok_or_else(|| Error::message(ERR_VMMF))
      .map(|fragment| once('#').chain(fragment.chars()).collect())
  }
}

impl<T> Deref for MethodWrap<'_, T> {
  type Target = Method<T>;

  fn deref(&self) -> &Self::Target {
    self.method
  }
}
