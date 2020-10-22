use alloc::string::String;
use alloc::vec::Vec;

use crate::error::Error;
use crate::error::Result;
use crate::utils::Object;

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum MethodData {
  PublicKeyBase58(String),
  PublicKeyHex(String),
  PublicKeyJwk(Object),
  PublicKeyPem(String),
}

impl MethodData {
  pub fn try_decode(&self) -> Result<Option<Vec<u8>>> {
    match self {
      Self::PublicKeyJwk(_) => Ok(None),
      Self::PublicKeyPem(_) => Ok(None), // TODO
      Self::PublicKeyBase58(inner) => bs58::decode(inner)
        .into_vec()
        .map_err(|_| Error::InvalidKey { error: "oh no" })
        .map(Some),
      Self::PublicKeyHex(inner) => hex::decode(inner)
        .map_err(|_| Error::InvalidKey { error: "oh no" })
        .map(Some),
    }
  }
}
