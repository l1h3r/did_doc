use crate::verifiable::Signature;
use crate::verifiable::SignatureDocument;

pub trait TrySignature {
  fn try_signature(&self) -> Option<&Signature>;
}

pub trait TrySignatureMut: TrySignature {
  fn try_signature_mut(&mut self) -> Option<&mut Signature>;
}

pub trait SetSignature: TrySignatureMut {
  fn set_signature(&mut self, signature: Signature);
}

impl<T> TrySignature for T
where
  T: SignatureDocument,
{
  fn try_signature(&self) -> Option<&Signature> {
    <T as SignatureDocument>::try_signature(self)
  }
}

impl<T> TrySignatureMut for T
where
  T: SignatureDocument,
{
  fn try_signature_mut(&mut self) -> Option<&mut Signature> {
    <T as SignatureDocument>::try_signature_mut(self)
  }
}

impl<T> SetSignature for T
where
  T: SignatureDocument,
{
  fn set_signature(&mut self, signature: Signature) {
    <T as SignatureDocument>::set_signature(self, signature)
  }
}
