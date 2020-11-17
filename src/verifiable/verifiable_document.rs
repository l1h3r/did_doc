use alloc::vec::Vec;
use core::fmt::Display;
use core::fmt::Formatter;
use core::fmt::Result as FmtResult;
use core::ops::Deref;
use core::ops::DerefMut;
use serde::Serialize;

use crate::document::Document;
use crate::error::Result;
use crate::utils::Object;
use crate::verifiable::DocumentReader;
use crate::verifiable::DocumentWriter;
use crate::verifiable::SetSignature;
use crate::verifiable::Signature;
use crate::verifiable::SignatureDocument;
use crate::verifiable::SignatureOptions;
use crate::verifiable::SignatureSuite;
use crate::verifiable::TrySignature;
use crate::verifiable::VerifiableProperties;
use crate::verification::MethodQuery;

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
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

  pub fn sign<'a, Q, S>(&mut self, suite: S, query: Q, secret: &[u8]) -> Result<()>
  where
    T: Serialize,
    U: Serialize,
    V: Serialize,
    S: SignatureSuite,
    Q: Into<MethodQuery<'a>>,
  {
    let options: SignatureOptions = self.signature_options(query)?;

    SignatureDocument::sign_data(self, suite, options, secret)?;

    Ok(())
  }

  pub fn verify<S>(&self, suite: S) -> Result<()>
  where
    T: Serialize,
    U: Serialize,
    V: Serialize,
    S: SignatureSuite,
  {
    SignatureDocument::verify_data(self, suite)?;

    Ok(())
  }

  pub fn sign_data<'a, D, Q, S>(
    &self,
    data: &mut D,
    suite: S,
    query: Q,
    secret: &[u8],
  ) -> Result<()>
  where
    D: Serialize + SetSignature,
    S: SignatureSuite,
    Q: Into<MethodQuery<'a>>,
  {
    let options: SignatureOptions = self.signature_options(query)?;

    DocumentWriter::new(data, self).sign(suite, options, secret)?;

    Ok(())
  }

  pub fn verify_data<D, S>(&self, data: &D, suite: S) -> Result<()>
  where
    D: Serialize + TrySignature,
    S: SignatureSuite,
  {
    DocumentReader::new(data, self).verify(suite)?;

    Ok(())
  }

  pub(crate) fn signature_options<'a, Q>(&self, query: Q) -> Result<SignatureOptions>
  where
    Q: Into<MethodQuery<'a>>,
  {
    self
      .document
      .try_resolve(query)
      .and_then(|method| method.try_into_fragment())
      .map(SignatureOptions::new)
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
  fn resolve_method(&self, method: &str) -> Option<Vec<u8>> {
    self.resolve_bytes(method)
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
