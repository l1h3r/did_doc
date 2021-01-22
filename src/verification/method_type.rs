use core::str::FromStr;

use crate::error::Error;
use crate::error::Result;

const ERR_UMT: &str = "Unknown Method Type";

/// Supported verification method types.
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Deserialize, Serialize)]
#[non_exhaustive]
pub enum MethodType {
  JcsEd25519Key2020,
  JwsVerificationKey2020,
  Ed25519VerificationKey2018,
  MerkleKeyCollection2021,
}

impl MethodType {
  pub const fn as_str(self) -> &'static str {
    match self {
      Self::JcsEd25519Key2020 => "JcsEd25519Key2020",
      Self::JwsVerificationKey2020 => "JwsVerificationKey2020",
      Self::Ed25519VerificationKey2018 => "Ed25519VerificationKey2018",
      Self::MerkleKeyCollection2021 => "MerkleKeyCollection2021",
    }
  }
}

impl FromStr for MethodType {
  type Err = Error;

  fn from_str(string: &str) -> Result<Self, Self::Err> {
    match string {
      "JcsEd25519Key2020" => Ok(Self::JcsEd25519Key2020),
      "JwsVerificationKey2020" => Ok(Self::JwsVerificationKey2020),
      "Ed25519VerificationKey2018" => Ok(Self::Ed25519VerificationKey2018),
      "MerkleKeyCollection2021" => Ok(Self::MerkleKeyCollection2021),
      _ => Err(Error::message(ERR_UMT)),
    }
  }
}
