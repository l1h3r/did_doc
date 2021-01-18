use serde::Serialize;

use crate::error::Error;
use crate::error::Result;
use crate::lib::*;
use crate::verifiable::Signature;
use crate::verifiable::SignatureData;
use crate::verifiable::SignatureOptions;
use crate::verifiable::SignatureSuite;
use crate::verification::MethodIndex;
use crate::verification::MethodQuery;

const ERR_VMNF: &str = "Verification Method Not Found";
const ERR_SNF: &str = "Signature Not Found";

pub trait SignatureDocument: Serialize + Sized {
  fn resolve_method(&self, query: MethodQuery) -> Option<Vec<u8>>;

  fn try_signature(&self) -> Option<&Signature>;

  fn try_signature_mut(&mut self) -> Option<&mut Signature>;

  fn set_signature(&mut self, signature: Signature);

  fn sign_doc<T>(&mut self, suite: T, options: SignatureOptions, secret: &[u8]) -> Result<()>
  where
    T: SignatureSuite,
  {
    self.set_signature(Signature::new(suite.name(), options));

    let value: SignatureData = suite.sign(self, secret)?;

    self.signature_mut()?.data_mut().set(value);

    Ok(())
  }

  fn verify_doc<T>(&self, suite: T) -> Result<()>
  where
    T: SignatureSuite,
  {
    let signature: &Signature = self.signature()?;
    let identifier: MethodIndex = MethodIndex::Ident(&signature.verification_method);

    let query: MethodQuery = if let Some(purpose) = signature.proof_purpose.as_deref() {
      MethodQuery::with_scope(identifier, purpose.parse()?)
    } else {
      MethodQuery::new(identifier)
    };

    let method: Vec<u8> = self
      .resolve_method(query)
      .ok_or_else(|| Error::message(ERR_VMNF))?;

    signature.hide_value();

    suite.verify(self, signature.data(), &method)?;

    signature.show_value();

    Ok(())
  }

  fn signature(&self) -> Result<&Signature, Error> {
    self.try_signature().ok_or_else(|| Error::message(ERR_SNF))
  }

  fn signature_mut(&mut self) -> Result<&mut Signature, Error> {
    self
      .try_signature_mut()
      .ok_or_else(|| Error::message(ERR_SNF))
  }
}
