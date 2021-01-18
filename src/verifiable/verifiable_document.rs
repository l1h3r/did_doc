use core::fmt::Debug;
use core::fmt::Display;
use core::fmt::Formatter;
use core::fmt::Result as FmtResult;
use core::ops::Deref;
use core::ops::DerefMut;
use serde::Serialize;

use crate::document::Document;
use crate::signature::Signature;
use crate::utils::Object;
use crate::verifiable::ResolveMethod;
use crate::verifiable::SetSignature;
use crate::verifiable::TrySignature;
use crate::verifiable::TrySignatureMut;
use crate::verifiable::VerifiableProperties;
use crate::verification::MethodQuery;
use crate::verification::MethodWrap;

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

  pub fn set_proof(&mut self, signature: Signature) {
    self.properties_mut().proof = Some(signature);
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

impl<T, U, V> TrySignature for VerifiableDocument<T, U, V> {
  fn signature(&self) -> Option<&Signature> {
    self.proof()
  }
}

impl<T, U, V> TrySignatureMut for VerifiableDocument<T, U, V> {
  fn signature_mut(&mut self) -> Option<&mut Signature> {
    self.proof_mut()
  }
}

impl<T, U, V> SetSignature for VerifiableDocument<T, U, V> {
  fn set_signature(&mut self, signature: Signature) {
    self.set_proof(signature)
  }
}

impl<T, U, V> ResolveMethod<U> for VerifiableDocument<T, U, V> {
  fn resolve_method(&self, query: MethodQuery<'_>) -> Option<MethodWrap<'_, U>> {
    self.document.resolve(query)
  }
}
