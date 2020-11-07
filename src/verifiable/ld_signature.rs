use serde::Serialize;

use crate::error::Result;
use crate::verifiable::SignatureData;

pub trait LdSignature {
  fn name(&self) -> &'static str;

  fn sign<T>(&self, message: &T, secret: &[u8]) -> Result<SignatureData>
  where
    T: Serialize;

  fn verify<T>(&self, message: &T, signature: &SignatureData, public: &[u8]) -> Result<()>
  where
    T: Serialize;
}
