use crate::verification::MethodIndex;
use crate::verification::MethodScope;

/// Specifies the  conditions of a DID document method resolution query.
///
/// See `Document::resolve`.
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct MethodQuery<'a> {
  pub(crate) ident: MethodIndex<'a>,
  pub(crate) scope: MethodScope,
}

impl<'a> MethodQuery<'a> {
  /// Creates a new `MethodQuery`.
  pub fn new<T>(ident: T) -> Self
  where
    T: Into<MethodIndex<'a>>,
  {
    Self::with_scope(ident, MethodScope::default())
  }

  /// Creates a new `MethodQuery` with the given `MethodScope`.
  pub fn with_scope<T>(ident: T, scope: MethodScope) -> Self
  where
    T: Into<MethodIndex<'a>>,
  {
    Self {
      ident: ident.into(),
      scope,
    }
  }
}

impl<'a> From<&'a str> for MethodQuery<'a> {
  fn from(other: &'a str) -> Self {
    Self::new(other)
  }
}

impl From<usize> for MethodQuery<'_> {
  fn from(other: usize) -> Self {
    Self::new(other)
  }
}

impl<'a> From<(&'a str, MethodScope)> for MethodQuery<'a> {
  fn from(other: (&'a str, MethodScope)) -> Self {
    Self::with_scope(other.0, other.1)
  }
}

impl From<(usize, MethodScope)> for MethodQuery<'_> {
  fn from(other: (usize, MethodScope)) -> Self {
    Self::with_scope(other.0, other.1)
  }
}

impl<'a> From<(MethodIndex<'a>, MethodScope)> for MethodQuery<'a> {
  fn from(other: (MethodIndex<'a>, MethodScope)) -> Self {
    Self::with_scope(other.0, other.1)
  }
}

impl<'a> From<MethodScope> for MethodQuery<'a> {
  fn from(other: MethodScope) -> Self {
    Self::with_scope(0, other)
  }
}
