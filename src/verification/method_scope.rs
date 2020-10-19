#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum MethodScope {
  VerificationMethod,
  Authentication,
  AssertionMethod,
  KeyAgreement,
  CapabilityDelegation,
  CapabilityInvocation,
}

impl Default for MethodScope {
  fn default() -> Self {
    Self::VerificationMethod
  }
}
