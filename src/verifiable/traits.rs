use crate::error::Error;
use crate::error::Result;
use crate::signature::Signature;
use crate::verification::MethodQuery;
use crate::verification::MethodWrap;

const ERR_VMNF: &str = "Verification Method Not Found";
const ERR_SNF: &str = "Signature Not Found";

pub trait TrySignature {
  fn signature(&self) -> Option<&Signature>;

  fn try_signature(&self) -> Result<&Signature> {
    self.signature().ok_or_else(|| Error::message(ERR_SNF))
  }
}

impl<'a, T> TrySignature for &'a T
where
  T: TrySignature,
{
  fn signature(&self) -> Option<&Signature> {
    (**self).signature()
  }
}

impl<'a, T> TrySignature for &'a mut T
where
  T: TrySignature,
{
  fn signature(&self) -> Option<&Signature> {
    (**self).signature()
  }
}

// =============================================================================
// =============================================================================

pub trait TrySignatureMut: TrySignature {
  fn signature_mut(&mut self) -> Option<&mut Signature>;

  fn try_signature_mut(&mut self) -> Result<&mut Signature> {
    self.signature_mut().ok_or_else(|| Error::message(ERR_SNF))
  }
}

impl<'a, T> TrySignatureMut for &'a mut T
where
  T: TrySignatureMut,
{
  fn signature_mut(&mut self) -> Option<&mut Signature> {
    (**self).signature_mut()
  }
}

// =============================================================================
// =============================================================================

pub trait SetSignature: TrySignatureMut {
  fn set_signature(&mut self, signature: Signature);
}

impl<'a, T> SetSignature for &'a mut T
where
  T: SetSignature,
{
  fn set_signature(&mut self, signature: Signature) {
    (**self).set_signature(signature);
  }
}

// =============================================================================
// =============================================================================

pub trait ResolveMethod<M> {
  fn resolve_method(&self, query: MethodQuery<'_>) -> Option<MethodWrap<'_, M>>;

  fn try_resolve_method(&self, query: MethodQuery<'_>) -> Result<MethodWrap<'_, M>> {
    self
      .resolve_method(query)
      .ok_or_else(|| Error::message(ERR_VMNF))
  }
}

impl<'a, T, M> ResolveMethod<M> for &'a T
where
  T: ResolveMethod<M>,
{
  fn resolve_method(&self, query: MethodQuery<'_>) -> Option<MethodWrap<'_, M>> {
    (**self).resolve_method(query)
  }
}
