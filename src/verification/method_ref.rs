use did_url::DID;

use crate::verification::Method;

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(untagged)]
pub enum MethodRef {
  Embed(Method),
  Refer(DID),
}

impl MethodRef {
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

impl From<Method> for MethodRef {
  fn from(other: Method) -> Self {
    Self::Embed(other)
  }
}

impl From<DID> for MethodRef {
  fn from(other: DID) -> Self {
    Self::Refer(other)
  }
}
