use alloc::vec::Vec;
use serde::Serialize;

use crate::verifiable::LdDocument;
use crate::verifiable::Signature;
use crate::verifiable::TrySignature;
use crate::verifiable::VerifiableDocument;

#[derive(Debug, Serialize)]
pub struct LdReader<'a, 'b, T, U> {
  #[serde(skip)]
  root: &'b VerifiableDocument<U>,
  #[serde(flatten)]
  data: &'a T,
}

impl<'a, 'b, T, U> LdReader<'a, 'b, T, U> {
  pub fn new(root: &'b VerifiableDocument<U>, data: &'a T) -> Self {
    Self { root, data }
  }
}

impl<T, U> LdDocument for LdReader<'_, '_, T, U>
where
  T: Serialize + TrySignature,
  U: Serialize,
{
  fn resolve_method(&self, method: &str) -> Option<Vec<u8>> {
    LdDocument::resolve_method(self.root, method)
  }

  fn try_signature(&self) -> Option<&Signature> {
    self.data.try_signature()
  }

  fn try_signature_mut(&mut self) -> Option<&mut Signature> {
    None
  }

  fn set_signature(&mut self, _signature: Signature) {
    // do nothing
  }
}
