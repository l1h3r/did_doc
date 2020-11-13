use serde::Serialize;

use crate::error::Result;
use crate::verifiable::SignatureData;

pub trait SignatureSuite {
  fn name(&self) -> &'static str;

  fn sign<M>(&self, message: &M, secret: &[u8]) -> Result<SignatureData>
  where
    M: Serialize;

  fn verify<M>(&self, message: &M, signature: &SignatureData, public: &[u8]) -> Result<()>
  where
    M: Serialize;
}

impl<T> SignatureSuite for &'_ T
where
  T: SignatureSuite,
{
  fn name(&self) -> &'static str {
    T::name(&**self)
  }

  fn sign<M>(&self, message: &M, secret: &[u8]) -> Result<SignatureData>
  where
    M: Serialize,
  {
    T::sign(&**self, message, secret)
  }

  fn verify<M>(&self, message: &M, signature: &SignatureData, public: &[u8]) -> Result<()>
  where
    M: Serialize,
  {
    T::verify(&**self, message, signature, public)
  }
}
