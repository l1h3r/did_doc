use alloc::vec::Vec;
use core::fmt::Display;
use core::fmt::Debug;
use core::fmt::Formatter;
use core::fmt::Result as FmtResult;
use core::ops::Deref;
use core::ops::DerefMut;
use serde::Serialize;

use crate::document::Document;
use crate::error::Result;
use crate::utils::Object;
use crate::verifiable::Signature;
use crate::verifiable::SignatureDocument;
use crate::verifiable::SignatureOptions;
use crate::verifiable::SignatureSuite;
use crate::verifiable::VerifiableProperties;
use crate::verification::MethodQuery;

#[derive(Clone, PartialEq, Deserialize, Serialize)]
#[repr(transparent)]
#[serde(transparent)]
pub struct VerifiableDocument<T = Object, U = Object, V = Object> {
  document: Document<VerifiableProperties<T>, U, V>,
}

impl<T, U, V> VerifiableDocument<T, U, V> {
  pub fn new(document: Document<T, U, V>) -> Self {
    Self {
      document: document.map(VerifiableProperties::new),
    }
  }

  pub fn with_proof(document: Document<T, U, V>, proof: Signature) -> Self {
    Self {
      document: document.map(|old| VerifiableProperties::with_proof(old, proof)),
    }
  }

  pub fn proof(&self) -> Option<&Signature> {
    self.properties().proof()
  }

  pub fn proof_mut(&mut self) -> Option<&mut Signature> {
    self.properties_mut().proof_mut()
  }
}

impl<T, U, V> VerifiableDocument<T, U, V>
where
  T: Serialize,
  U: Serialize,
  V: Serialize,
{
  pub fn sign<S>(&mut self, suite: S, options: SignatureOptions, secret: &[u8]) -> Result<()>
  where
    S: SignatureSuite,
  {
    self.sign_doc(suite, options, secret)
  }

  pub fn verify<S>(&self, suite: S) -> Result<()>
  where
    S: SignatureSuite,
  {
    self.verify_doc(suite)
  }
}

impl<T, U, V> Deref for VerifiableDocument<T, U, V> {
  type Target = Document<VerifiableProperties<T>, U, V>;

  fn deref(&self) -> &Self::Target {
    &self.document
  }
}

impl<T, U, V> DerefMut for VerifiableDocument<T, U, V> {
  fn deref_mut(&mut self) -> &mut Self::Target {
    &mut self.document
  }
}

impl<T, U, V> Debug for VerifiableDocument<T, U, V>
where
  T: Debug,
  U: Debug,
  V: Debug,
{
  fn fmt(&self, f: &mut Formatter) -> FmtResult {
    Debug::fmt(&self.document, f)
  }
}

impl<T, U, V> Display for VerifiableDocument<T, U, V>
where
  T: Serialize,
  U: Serialize,
  V: Serialize,
{
  fn fmt(&self, f: &mut Formatter) -> FmtResult {
    Display::fmt(&self.document, f)
  }
}

impl<T, U, V> SignatureDocument for VerifiableDocument<T, U, V>
where
  T: Serialize,
  U: Serialize,
  V: Serialize,
{
  fn resolve_method(&self, query: MethodQuery) -> Option<Vec<u8>> {
    self.resolve_bytes(query)
  }

  fn try_signature(&self) -> Option<&Signature> {
    self.document.properties().proof()
  }

  fn try_signature_mut(&mut self) -> Option<&mut Signature> {
    self.document.properties_mut().proof_mut()
  }

  fn set_signature(&mut self, signature: Signature) {
    self.document.properties_mut().proof = Some(signature);
  }
}
