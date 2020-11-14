use did_url::DID;

use crate::utils::Object;
use crate::verification::Method;

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(untagged)]
pub enum MethodRef<T = Object> {
  Embed(Method<T>),
  Refer(DID),
}

impl<T> MethodRef<T> {
  /// Returns a reference to the `MethodRef` id.
  pub fn id(&self) -> &DID {
    match self {
      Self::Embed(inner) => inner.id(),
      Self::Refer(inner) => inner,
    }
  }

  /// Returns a reference to the `MethodRef` controller.
  pub fn controller(&self) -> Option<&DID> {
    match self {
      Self::Embed(inner) => Some(inner.controller()),
      Self::Refer(_) => None,
    }
  }

  /// Returns a `bool` indicating if the `MethodRef` is an embedded `Method`.
  #[inline]
  pub const fn is_embedded(&self) -> bool {
    matches!(self, Self::Embed(_))
  }

  /// Returns a `bool` indicating if the `MethodRef` is a `DID` reference.
  #[inline]
  pub const fn is_referred(&self) -> bool {
    matches!(self, Self::Refer(_))
  }

  /// Returns the inner `Method` if this is an embedded `MethodRef`.
  ///
  /// Note: Returns `Err(self)` as a failure case.
  ///
  /// # Errors
  ///
  /// Fails if `MethodRef` is not an embedded method.
  pub fn try_into_embedded(self) -> Result<Method<T>, Self> {
    match self {
      Self::Embed(inner) => Ok(inner),
      Self::Refer(_) => Err(self),
    }
  }

  /// Returns the inner `Method` if this is an referenced `MethodRef`.
  ///
  /// Note: Returns `Err(self)` as a failure case.
  ///
  /// # Errors
  ///
  /// Fails if `MethodRef` is not an referenced method.
  pub fn try_into_referenced(self) -> Result<DID, Self> {
    match self {
      Self::Embed(_) => Err(self),
      Self::Refer(inner) => Ok(inner),
    }
  }
}

impl<T> From<Method<T>> for MethodRef<T> {
  #[inline]
  fn from(other: Method<T>) -> Self {
    Self::Embed(other)
  }
}

impl<T> From<DID> for MethodRef<T> {
  #[inline]
  fn from(other: DID) -> Self {
    Self::Refer(other)
  }
}

impl<T> AsRef<DID> for MethodRef<T> {
  #[inline]
  fn as_ref(&self) -> &DID {
    self.id()
  }
}
