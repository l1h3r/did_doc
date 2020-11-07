use alloc::string::String;
use alloc::vec::Vec;

use crate::error::Error;
use crate::error::Result;
use crate::utils::Object;

const ERR_IKD: &str = "Invalid Key Data";
const ERR_IB16: &str = "Invalid Base16 Key Data";
const ERR_IB58: &str = "Invalid Base58 Key Data";

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
      Self::PublicKeyBase58(input) => decode_b58(input),
      Self::PublicKeyHex(input) => decode_hex(input),
      Self::PublicKeyJwk(_) => Err(Error::message(ERR_IKD)),
    }
  }
}

fn decode_hex(input: &str) -> Result<Vec<u8>> {
  hex::decode(input).map_err(|_| Error::message(ERR_IB16))
}

fn decode_b58(input: &str) -> Result<Vec<u8>> {
  bs58::decode(input)
    .into_vec()
    .map_err(|_| Error::message(ERR_IB58))
}
