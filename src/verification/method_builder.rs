use did_url::DID;

use crate::lib::*;
use crate::error::Result;
use crate::utils::Object;
use crate::utils::Value;
use crate::verification::Method;
use crate::verification::MethodData;
use crate::verification::MethodType;

/// A `MethodBuilder` is used to generate a customized `Method`.
#[derive(Clone, Debug, Default)]
pub struct MethodBuilder<T = Object> {
  pub(crate) id: Option<DID>,
  pub(crate) controller: Option<DID>,
  pub(crate) key_type: Option<MethodType>,
  pub(crate) key_data: Option<MethodData>,
  pub(crate) properties: T,
}

impl<T> MethodBuilder<T> {
  /// Creates a new `MethodBuilder`.
  pub fn new(properties: T) -> Self {
    Self {
      id: None,
      controller: None,
      key_type: None,
      key_data: None,
      properties,
    }
  }

  /// Sets the `id` value of the generated verification `Method`.
  #[must_use]
  pub fn id(mut self, value: DID) -> Self {
    self.id = Some(value);
    self
  }

  /// Sets the `controller` value of the generated verification `Method`.
  #[must_use]
  pub fn controller(mut self, value: DID) -> Self {
    self.controller = Some(value);
    self
  }

  /// Sets the `type` value of the generated verification `Method`.
  #[must_use]
  pub fn key_type(mut self, value: MethodType) -> Self {
    self.key_type = Some(value);
    self
  }

  /// Sets the `data` value of the generated verification `Method`.
  #[must_use]
  pub fn key_data(mut self, value: MethodData) -> Self {
    self.key_data = Some(value);
    self
  }

  /// Returns a new `Method` based on the `MethodBuilder` configuration.
  pub fn build(self) -> Result<Method<T>> {
    Method::from_builder(self)
  }
}

impl MethodBuilder {
  /// Adds a new custom property to the generated `Method`.
  #[must_use]
  pub fn property<K, V>(mut self, key: K, value: V) -> Self
  where
    K: Into<String>,
    V: Into<Value>,
  {
    self.properties.insert(key.into(), value.into());
    self
  }

  /// Adds a series of custom properties to the generated `Method`.
  #[must_use]
  pub fn properties<K, V, I>(mut self, iter: I) -> Self
  where
    I: IntoIterator<Item = (K, V)>,
    K: Into<String>,
    V: Into<Value>,
  {
    self
      .properties
      .extend(iter.into_iter().map(|(k, v)| (k.into(), v.into())));
    self
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  #[should_panic = "Missing `id`"]
  fn test_missing_id() {
    let _: Method = MethodBuilder::default()
      .controller("did:example:123".parse().unwrap())
      .key_type(MethodType::Ed25519VerificationKey2018)
      .key_data(MethodData::PublicKeyBase58("".into()))
      .build()
      .unwrap();
  }

  #[test]
  #[should_panic = "Missing `key_type`"]
  fn test_missing_key_type() {
    let _: Method = MethodBuilder::default()
      .id("did:example:123".parse().unwrap())
      .controller("did:example:123".parse().unwrap())
      .key_data(MethodData::PublicKeyBase58("".into()))
      .build()
      .unwrap();
  }

  #[test]
  #[should_panic = "Missing `key_data`"]
  fn test_missing_key_data() {
    let _: Method = MethodBuilder::default()
      .id("did:example:123".parse().unwrap())
      .controller("did:example:123".parse().unwrap())
      .key_type(MethodType::Ed25519VerificationKey2018)
      .build()
      .unwrap();
  }

  #[test]
  #[should_panic = "Missing `controller`"]
  fn test_missing_controller() {
    let _: Method = MethodBuilder::default()
      .id("did:example:123".parse().unwrap())
      .key_type(MethodType::Ed25519VerificationKey2018)
      .key_data(MethodData::PublicKeyBase58("".into()))
      .build()
      .unwrap();
  }
}
