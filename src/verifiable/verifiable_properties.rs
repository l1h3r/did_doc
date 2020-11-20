use core::ops::Deref;
use core::ops::DerefMut;

use crate::utils::Object;
use crate::verifiable::Signature;

#[derive(Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Deserialize, Serialize)]
pub struct VerifiableProperties<T = Object> {
  #[serde(flatten)]
  pub(crate) properties: T,
  // TODO: Support multiple signatures (?)
  #[serde(skip_serializing_if = "Option::is_none")]
  pub(crate) proof: Option<Signature>,
}

impl<T> VerifiableProperties<T> {
  pub const fn new(properties: T) -> Self {
    Self {
      properties,
      proof: None,
    }
  }

  pub const fn with_proof(properties: T, proof: Signature) -> Self {
    Self {
      properties,
      proof: Some(proof),
    }
  }

  pub fn proof(&self) -> Option<&Signature> {
    self.proof.as_ref()
  }

  pub fn proof_mut(&mut self) -> Option<&mut Signature> {
    self.proof.as_mut()
  }
}

impl<T> Deref for VerifiableProperties<T> {
  type Target = T;

  fn deref(&self) -> &Self::Target {
    &self.properties
  }
}

impl<T> DerefMut for VerifiableProperties<T> {
  fn deref_mut(&mut self) -> &mut Self::Target {
    &mut self.properties
  }
}
