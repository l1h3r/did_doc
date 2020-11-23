use alloc::string::String;
use alloc::vec::Vec;

use crate::error::Error;
use crate::error::Result;
use crate::utils::Object;

const ERR_IKD: &str = "Invalid Key Data";
const ERR_IB16: &str = "Invalid Base16 Key Data";
const ERR_IB58: &str = "Invalid Base58 Key Data";

/// Supported verification method data formats.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub enum MethodData {
  PublicKeyBase58(String),
  PublicKeyHex(String),
  PublicKeyJwk(Object),
}

impl MethodData {
  /// Creates a new `MethodData` variant with base16-encoded content.
  pub fn new_b16(data: impl AsRef<[u8]>) -> Self {
    Self::PublicKeyHex(hex::encode(data.as_ref()))
  }

  /// Creates a new `MethodData` variant with base58-encoded content.
  pub fn new_b58(data: impl AsRef<[u8]>) -> Self {
    Self::PublicKeyBase58(bs58::encode(data.as_ref()).into_string())
  }

  /// Returns a `Vec<u8>` containing the decoded bytes of the `MethodData`.
  ///
  /// This is generally a public key identified by a `MethodType` value.
  ///
  /// # Errors
  ///
  /// Decoding can fail if `MethodData` has invalid content or cannot be
  /// represented as a vector of bytes.
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
