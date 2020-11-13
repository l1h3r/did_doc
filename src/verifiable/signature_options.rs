use alloc::string::String;

#[derive(Clone, Debug, Default, Hash, PartialEq, Eq, PartialOrd, Ord, Deserialize, Serialize)]
pub struct SignatureOptions {
  #[serde(rename = "verificationMethod")]
  pub verification_method: String,
  #[serde(rename = "proofPurpose", skip_serializing_if = "Option::is_none")]
  pub proof_purpose: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub created: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub nonce: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub domain: Option<String>,
}

impl SignatureOptions {
  pub const fn new(verification_method: String) -> Self {
    Self {
      verification_method,
      proof_purpose: None,
      created: None,
      nonce: None,
      domain: None,
    }
  }
}
