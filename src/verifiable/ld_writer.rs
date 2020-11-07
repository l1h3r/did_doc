use alloc::vec::Vec;
use serde::Serialize;

use crate::verifiable::LdDocument;
use crate::verifiable::SetSignature;
use crate::verifiable::Signature;
use crate::verifiable::VerifiableDocument;

#[derive(Debug, Serialize)]
pub struct LdWriter<'a, 'b, T, U> {
  #[serde(skip)]
  root: &'b VerifiableDocument<U>,
  #[serde(flatten)]
  data: &'a mut T,
}

impl<'a, 'b, T, U> LdWriter<'a, 'b, T, U> {
  pub fn new(root: &'b VerifiableDocument<U>, data: &'a mut T) -> Self {
    Self { root, data }
  }
}

impl<T, U> LdDocument for LdWriter<'_, '_, T, U>
where
  T: Serialize + SetSignature,
  U: Serialize,
{
  fn resolve_method(&self, method: &str) -> Option<Vec<u8>> {
    LdDocument::resolve_method(self.root, method)
  }

  fn try_signature(&self) -> Option<&Signature> {
    self.data.try_signature()
  }

  fn try_signature_mut(&mut self) -> Option<&mut Signature> {
    self.data.try_signature_mut()
  }

  fn set_signature(&mut self, signature: Signature) {
    self.data.set_signature(signature);
  }
}
