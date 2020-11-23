use core::ops::Deref;

use crate::utils::Object;
use crate::verification::Method;
use crate::verification::MethodScope;

/// A queried `Method` with additional metadata about the query resolution.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MethodWrap<'a, T = Object> {
  pub(crate) method: &'a Method<T>,
  pub(crate) index: usize,
  pub(crate) scope: MethodScope,
}

impl<'a, T> MethodWrap<'a, T> {
  /// Creates a new `MethodWrap`.
  pub(crate) const fn new(method: &'a Method<T>, index: usize, scope: MethodScope) -> Self {
    Self {
      index,
      method,
      scope,
    }
  }

  /// Returns the index of the method within the verification relationship set.
  pub const fn index(&self) -> usize {
    self.index
  }

  /// Returns the scope of the resolved verification method.
  pub const fn scope(&self) -> MethodScope {
    self.scope
  }

  /// Consumes the `MethodWrap` and returns a reference to the resolved `Method`.
  pub const fn into_method(self) -> &'a Method<T> {
    self.method
  }
}

impl<T> Deref for MethodWrap<'_, T> {
  type Target = Method<T>;

  fn deref(&self) -> &Self::Target {
    self.method
  }
}
