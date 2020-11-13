use alloc::vec::Vec;
use serde::Serialize;

use crate::verifiable::SetSignature;
use crate::verifiable::Signature;
use crate::verifiable::SignatureDocument;
use crate::verifiable::VerifiableDocument;

#[derive(Debug, Serialize)]
pub struct SignatureWriter<'a, 'b, D, T, U, V> {
  #[serde(skip)]
  root: &'b VerifiableDocument<T, U, V>,
  #[serde(flatten)]
  data: &'a mut D,
}

impl<'a, 'b, D, T, U, V> SignatureWriter<'a, 'b, D, T, U, V> {
  pub fn new(root: &'b VerifiableDocument<T, U, V>, data: &'a mut D) -> Self {
    Self { root, data }
  }
}

impl<D, T, U, V> SignatureDocument for SignatureWriter<'_, '_, D, T, U, V>
where
  D: Serialize + SetSignature,
{
  fn resolve_method(&self, method: &str) -> Option<Vec<u8>> {
    self.root._resolve(method)
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
