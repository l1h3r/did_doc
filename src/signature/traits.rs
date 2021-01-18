use serde::Serialize;

use crate::error::Result;
use crate::lib::*;
use crate::signature::SignatureData;
use crate::verification::Method;

pub trait SuiteName {
  fn name(&self) -> String;
}

impl<'a, T> SuiteName for &'a T
where
  T: SuiteName,
{
  fn name(&self) -> String {
    (**self).name()
  }
}

// =============================================================================
// =============================================================================

pub trait Sign {
  fn sign<T>(&self, data: &T, secret: &[u8]) -> Result<SignatureData>
  where
    T: Serialize;
}

impl<'a, T> Sign for &'a T
where
  T: Sign,
{
  fn sign<U>(&self, data: &U, secret: &[u8]) -> Result<SignatureData>
  where
    U: Serialize,
  {
    (**self).sign(data, secret)
  }
}

// =============================================================================
// =============================================================================

pub trait Verify {
  fn verify<T, U>(&self, data: &T, signature: &SignatureData, method: &Method<U>) -> Result<()>
  where
    T: Serialize,
    U: Serialize;
}

impl<'a, T> Verify for &'a T
where
  T: Verify,
{
  fn verify<U, V>(&self, data: &U, signature: &SignatureData, method: &Method<V>) -> Result<()>
  where
    U: Serialize,
    V: Serialize,
  {
    (**self).verify(data, signature, method)
  }
}
