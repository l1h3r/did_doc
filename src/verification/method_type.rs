use core::str::FromStr;

use crate::error::Error;
use crate::error::Result;

const ERR_UMT: &str = "Unknown Method Type";

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Deserialize, Serialize)]
#[non_exhaustive]
pub enum MethodType {
  JcsEd25519Key2020,
  JwsVerificationKey2020,
  Ed25519VerificationKey2018,
}

impl FromStr for MethodType {
  type Err = Error;

  fn from_str(string: &str) -> Result<Self, Self::Err> {
    match string {
      "JcsEd25519Key2020" => Ok(Self::JcsEd25519Key2020),
      "JwsVerificationKey2020" => Ok(Self::JwsVerificationKey2020),
      "Ed25519VerificationKey2018" => Ok(Self::Ed25519VerificationKey2018),
      _ => Err(Error::message(ERR_UMT)),
    }
  }
}
