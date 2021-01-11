use serde::Serialize;

use crate::lib::*;
use crate::document::Document;
use crate::error::Result;
use crate::utils::Object;
use crate::verifiable::SetSignature;
use crate::verifiable::Signature;
use crate::verifiable::SignatureDocument;
use crate::verifiable::SignatureOptions;
use crate::verifiable::SignatureSuite;
use crate::verification::MethodQuery;

#[derive(Debug, Serialize)]
pub struct DocumentWriter<'a, 'b, D, T = Object, U = Object, V = Object> {
  #[serde(flatten)]
  data: &'a mut D,
  #[serde(skip)]
  root: &'b Document<T, U, V>,
}

impl<'a, 'b, D, T, U, V> DocumentWriter<'a, 'b, D, T, U, V> {
  pub fn new(data: &'a mut D, root: &'b Document<T, U, V>) -> Self {
    Self { data, root }
  }

  pub fn sign<S>(&mut self, suite: S, options: SignatureOptions, secret: &[u8]) -> Result<()>
  where
    D: Serialize + SetSignature,
    S: SignatureSuite,
  {
    self.sign_doc(suite, options, secret)
  }
}

impl<D, T, U, V> SignatureDocument for DocumentWriter<'_, '_, D, T, U, V>
where
  D: Serialize + SetSignature,
{
  fn resolve_method(&self, query: MethodQuery) -> Option<Vec<u8>> {
    self.root.resolve_bytes(query)
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
