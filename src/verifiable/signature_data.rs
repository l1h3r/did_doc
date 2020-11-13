use alloc::string::String;

#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Deserialize, Serialize)]
pub enum SignatureData {
  #[serde(skip)]
  None,
  #[serde(rename = "jws")]
  Jws(String),
  #[serde(rename = "proofValue")]
  Proof(String),
  #[serde(rename = "signatureValue")]
  Signature(String),
}

impl SignatureData {
  pub const fn is_none(&self) -> bool {
    matches!(self, Self::None)
  }

  pub const fn is_jws(&self) -> bool {
    matches!(self, Self::Jws(_))
  }

  pub const fn is_proof(&self) -> bool {
    matches!(self, Self::Proof(_))
  }

  pub const fn is_signature(&self) -> bool {
    matches!(self, Self::Signature(_))
  }

  pub fn as_str(&self) -> &str {
    match self {
      SignatureData::None => "",
      SignatureData::Jws(inner) => &*inner,
      SignatureData::Proof(inner) => &*inner,
      SignatureData::Signature(inner) => &*inner,
    }
  }

  pub fn try_jws(&self) -> Option<&str> {
    match self {
      SignatureData::None => None,
      SignatureData::Jws(inner) => Some(&*inner),
      SignatureData::Proof(_) => None,
      SignatureData::Signature(_) => None,
    }
  }

  pub fn try_proof(&self) -> Option<&str> {
    match self {
      SignatureData::None => None,
      SignatureData::Jws(_) => None,
      SignatureData::Proof(inner) => Some(&*inner),
      SignatureData::Signature(_) => None,
    }
  }

  pub fn try_signature(&self) -> Option<&str> {
    match self {
      SignatureData::None => None,
      SignatureData::Jws(_) => None,
      SignatureData::Proof(_) => None,
      SignatureData::Signature(inner) => Some(&*inner),
    }
  }
}
