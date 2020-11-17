use alloc::vec::Vec;
use serde::Serialize;

use crate::error::Result;
use crate::utils::Object;
use crate::verifiable::Signature;
use crate::verifiable::SignatureDocument;
use crate::verifiable::SignatureSuite;
use crate::verifiable::TrySignature;
use crate::verification::Method;

#[derive(Debug, Serialize)]
pub struct MethodReader<'a, 'b, T = Object, U = Object> {
  #[serde(flatten)]
  data: &'a T,
  #[serde(skip)]
  root: &'b Method<U>,
}

impl<'a, 'b, T, U> MethodReader<'a, 'b, T, U> {
  pub fn new(data: &'a T, root: &'b Method<U>) -> Self {
    Self { data, root }
  }

  pub fn verify<S>(&self, suite: S) -> Result<()>
  where
    T: Serialize + TrySignature,
    S: SignatureSuite,
  {
    self.verify_data(suite)
  }
}

impl<T, U> SignatureDocument for MethodReader<'_, '_, T, U>
where
  T: Serialize + TrySignature,
{
  fn resolve_method(&self, _method: &str) -> Option<Vec<u8>> {
    self.root.key_data().try_decode().ok()
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
