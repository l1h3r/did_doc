use crate::verifiable::Signature;

pub trait TrySignature {
  fn try_signature(&self) -> Option<&Signature>;
}

pub trait TrySignatureMut: TrySignature {
  fn try_signature_mut(&mut self) -> Option<&mut Signature>;
}

pub trait SetSignature: TrySignatureMut {
  fn set_signature(&mut self, signature: Signature);
}
