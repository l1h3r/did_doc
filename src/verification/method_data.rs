use alloc::string::String;
use alloc::vec::Vec;

use crate::error::Error;
use crate::error::Result;
use crate::utils::Object;

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub enum MethodData {
  PublicKeyBase58(String),
  PublicKeyHex(String),
  PublicKeyJwk(Object),
}

impl MethodData {
  pub fn try_decode(&self) -> Result<Vec<u8>> {
    match self {
      Self::PublicKeyBase58(inner) => bs58::decode(inner)
        .into_vec()
        .map_err(|_| Error::InvalidKey { error: "oh no" }),
      Self::PublicKeyHex(inner) => {
        hex::decode(inner).map_err(|_| Error::InvalidKey { error: "oh no" })
      }
      Self::PublicKeyJwk(_) => Err(Error::InvalidKey { error: "oh no" }),
    }
  }
}
