use alloc::vec::Vec;
use serde::Serialize;

use crate::document::Document;
use crate::error::Result;
use crate::utils::Object;
use crate::verifiable::Signature;
use crate::verifiable::SignatureDocument;
use crate::verifiable::SignatureSuite;
use crate::verifiable::TrySignature;

#[derive(Debug, Serialize)]
pub struct DocumentReader<'a, 'b, D, T = Object, U = Object, V = Object> {
  #[serde(flatten)]
  data: &'a D,
  #[serde(skip)]
  root: &'b Document<T, U, V>,
}

impl<'a, 'b, D, T, U, V> DocumentReader<'a, 'b, D, T, U, V> {
  pub fn new(data: &'a D, root: &'b Document<T, U, V>) -> Self {
    Self { data, root }
  }

  pub fn verify<S>(&self, suite: S) -> Result<()>
  where
    D: Serialize + TrySignature,
    S: SignatureSuite,
  {
    self.verify_doc(suite)
  }
}

impl<D, T, U, V> SignatureDocument for DocumentReader<'_, '_, D, T, U, V>
where
  D: Serialize + TrySignature,
{
  fn resolve_method(&self, method: &str) -> Option<Vec<u8>> {
    self.root.resolve_bytes(method)
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
