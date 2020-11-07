use alloc::vec::Vec;
use core::fmt::Display;
use core::fmt::Formatter;
use core::fmt::Result as FmtResult;
use core::iter::once;
use core::ops::Deref;
use core::ops::DerefMut;
use serde::Serialize;

use crate::document::Document;
use crate::error::Error;
use crate::error::Result;
use crate::utils::Object;
use crate::verifiable::LdDocument;
use crate::verifiable::LdSignature;
use crate::verifiable::Signature;
use crate::verifiable::SignatureOptions;
use crate::verifiable::VerifiableProperties;
use crate::verification::MethodQuery;
use crate::verification::MethodWrap;

const ERR_VMNF: &str = "Verification Method Not Found";
const ERR_VMMF: &str = "Verification Method Missing Fragment";

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[repr(transparent)]
#[serde(transparent)]
pub struct VerifiableDocument<T = Object> {
  document: Document<VerifiableProperties<T>>,
}

impl<T> VerifiableDocument<T> {
  pub fn new(document: Document<T>) -> Self {
    Self {
      document: document.map(VerifiableProperties::new),
    }
  }

  pub fn with_proof(document: Document<T>, proof: Signature) -> Self {
    Self {
      document: document.map(|old| VerifiableProperties::with_proof(old, proof)),
    }
  }

  pub fn sign<'a, S, Q>(&mut self, suite: &S, query: Q, secret: &[u8]) -> Result<()>
  where
    T: Serialize,
    S: LdSignature + ?Sized,
    Q: Into<MethodQuery<'a>>,
  {
    let method: MethodWrap<_> = self
      .document
      .resolve(query)
      .ok_or_else(|| Error::message(ERR_VMNF))?;

    let fragment: &str = method
      .id()
      .fragment()
      .ok_or_else(|| Error::message(ERR_VMMF))?;

    let options: SignatureOptions = SignatureOptions {
      verification_method: once('#').chain(fragment.chars()).collect(),
      proof_purpose: None,
      created: None,
      nonce: None,
      domain: None,
    };

    // TODO: Suite based on verification method type; needs to be customizable

    self.sign_data(suite, options, secret)?;

    Ok(())
  }

  pub fn verify<S>(&self, suite: &S) -> Result<()>
  where
    T: Serialize,
    S: LdSignature + ?Sized,
  {
    // TODO: Suite based on verification method type; needs to be customizable

    self.verify_data(suite)?;

    Ok(())
  }
}

impl<T> Deref for VerifiableDocument<T> {
  type Target = Document<VerifiableProperties<T>>;

  fn deref(&self) -> &Self::Target {
    &self.document
  }
}

impl<T> DerefMut for VerifiableDocument<T> {
  fn deref_mut(&mut self) -> &mut Self::Target {
    &mut self.document
  }
}

impl<T> Display for VerifiableDocument<T>
where
  T: Serialize,
{
  fn fmt(&self, f: &mut Formatter) -> FmtResult {
    Display::fmt(&self.document, f)
  }
}

impl<T> LdDocument for VerifiableDocument<T>
where
  T: Serialize,
{
  fn resolve_method(&self, method: &str) -> Option<Vec<u8>> {
    self.document.resolve(method)?.key_data().try_decode().ok()
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
