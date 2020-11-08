#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Deserialize, Serialize)]
#[non_exhaustive]
pub enum MethodType {
  JwsVerificationKey2020,
  Ed25519VerificationKey2018,
}
