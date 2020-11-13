use alloc::vec::Vec;
use serde::Serialize;

use crate::verifiable::Signature;
use crate::verifiable::SignatureDocument;
use crate::verifiable::TrySignature;
use crate::verifiable::VerifiableDocument;

#[derive(Debug, Serialize)]
pub struct SignatureReader<'a, 'b, D, T, U, V> {
  #[serde(skip)]
  root: &'b VerifiableDocument<T, U, V>,
  #[serde(flatten)]
  data: &'a D,
}

impl<'a, 'b, D, T, U, V> SignatureReader<'a, 'b, D, T, U, V> {
  pub fn new(root: &'b VerifiableDocument<T, U, V>, data: &'a D) -> Self {
    Self { root, data }
  }
}

impl<D, T, U, V> SignatureDocument for SignatureReader<'_, '_, D, T, U, V>
where
  D: Serialize + TrySignature,
{
  fn resolve_method(&self, method: &str) -> Option<Vec<u8>> {
    self.root._resolve(method)
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
