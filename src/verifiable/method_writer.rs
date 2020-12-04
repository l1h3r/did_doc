use alloc::vec::Vec;
use serde::Serialize;

use crate::error::Result;
use crate::utils::Object;
use crate::verifiable::SetSignature;
use crate::verifiable::Signature;
use crate::verifiable::SignatureDocument;
use crate::verifiable::SignatureOptions;
use crate::verifiable::SignatureSuite;
use crate::verification::Method;
use crate::verification::MethodQuery;

#[derive(Debug, Serialize)]
pub struct MethodWriter<'a, 'b, T = Object, U = Object> {
  #[serde(flatten)]
  data: &'a mut T,
  #[serde(skip)]
  root: &'b Method<U>,
}

impl<'a, 'b, T, U> MethodWriter<'a, 'b, T, U> {
  pub fn new(data: &'a mut T, root: &'b Method<U>) -> Self {
    Self { data, root }
  }

  pub fn sign<S>(&mut self, suite: S, options: SignatureOptions, secret: &[u8]) -> Result<()>
  where
    T: Serialize + SetSignature,
    S: SignatureSuite,
  {
    self.sign_doc(suite, options, secret)
  }
}

impl<T, U> SignatureDocument for MethodWriter<'_, '_, T, U>
where
  T: Serialize + SetSignature,
{
  fn resolve_method(&self, _: MethodQuery) -> Option<Vec<u8>> {
    self.root.key_data().try_decode().ok()
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
