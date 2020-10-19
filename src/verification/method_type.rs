#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Deserialize, Serialize)]
pub enum MethodType {
  JsonWebKey2020,
  Ed25519VerificationKey2018,
}
