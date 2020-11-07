use alloc::string::String;
use did_url::DID;

use crate::error::Error;
use crate::error::Result;
use crate::utils::Object;
use crate::utils::Value;
use crate::verification::Method;
use crate::verification::MethodData;
use crate::verification::MethodType;

const ERR_MI: &str = "Missing `id`";
const ERR_MC: &str = "Missing `controller`";
const ERR_MKT: &str = "Missing `key_type`";
const ERR_MKD: &str = "Missing `key_data`";

#[derive(Clone, Debug, Default)]
pub struct MethodBuilder<T = Object> {
  id: Option<DID>,
  controller: Option<DID>,
  key_type: Option<MethodType>,
  key_data: Option<MethodData>,
  properties: T,
}

impl<T> MethodBuilder<T> {
  pub fn new(properties: T) -> Self {
    Self {
      id: None,
      controller: None,
      key_type: None,
      key_data: None,
      properties,
    }
  }

  pub fn id(mut self, value: DID) -> Self {
    self.id = Some(value);
    self
  }

  pub fn controller(mut self, value: DID) -> Self {
    self.controller = Some(value);
    self
  }

  pub fn key_type(mut self, value: MethodType) -> Self {
    self.key_type = Some(value);
    self
  }

  pub fn key_data(mut self, value: MethodData) -> Self {
    self.key_data = Some(value);
    self
  }

  pub fn build(self) -> Result<Method<T>> {
    let id: DID = self.id.ok_or(Error::InvalidBuilder {
      name: "Method",
      error: ERR_MI,
    })?;

    let controller: DID = self.controller.ok_or(Error::InvalidBuilder {
      name: "Method",
      error: ERR_MC,
    })?;

    let key_type: MethodType = self.key_type.ok_or(Error::InvalidBuilder {
      name: "Method",
      error: ERR_MKT,
    })?;

    let key_data: MethodData = self.key_data.ok_or(Error::InvalidBuilder {
      name: "Method",
      error: ERR_MKD,
    })?;

    Ok(Method {
      id,
      controller,
      key_type,
      key_data,
      properties: self.properties,
    })
  }
}

impl MethodBuilder {
  pub fn property<K, V>(mut self, key: K, value: V) -> Self
  where
    K: Into<String>,
    V: Into<Value>,
  {
    self.properties.insert(key.into(), value.into());
    self
  }

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
