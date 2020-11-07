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
  pub fn id(&self) -> &DID {
    match self {
      Self::Embed(inner) => inner.id(),
      Self::Refer(inner) => &inner,
    }
  }

  pub fn controller(&self) -> Option<&DID> {
    match self {
      Self::Embed(inner) => Some(inner.controller()),
      Self::Refer(_) => None,
    }
  }
}

impl<T> From<Method<T>> for MethodRef<T> {
  fn from(other: Method<T>) -> Self {
    Self::Embed(other)
  }
}

impl<T> From<DID> for MethodRef<T> {
  fn from(other: DID) -> Self {
    Self::Refer(other)
  }
}

impl<T> AsRef<DID> for MethodRef<T> {
  fn as_ref(&self) -> &DID {
    self.id()
  }
}
