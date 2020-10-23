use alloc::string::String;
use did_url::DID;

use crate::error::Error;
use crate::error::Result;
use crate::utils::Object;
use crate::utils::Value;
use crate::verification::Method;
use crate::verification::MethodData;
use crate::verification::MethodType;

#[derive(Clone, Debug, Default)]
pub struct MethodBuilder {
  id: Option<DID>,
  controller: Option<DID>,
  key_type: Option<MethodType>,
  key_data: Option<MethodData>,
  properties: Object,
}

impl MethodBuilder {
  pub fn new() -> Self {
    Self {
      id: None,
      controller: None,
      key_type: None,
      key_data: None,
      properties: Object::new(),
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

  pub fn property<T, U>(mut self, key: T, value: U) -> Self
  where
    T: Into<String>,
    U: Into<Value>,
  {
    self.properties.insert(key.into(), value.into());
    self
  }

  pub fn properties<T, U, I>(mut self, iter: I) -> Self
  where
    I: IntoIterator<Item = (T, U)>,
    T: Into<String>,
    U: Into<Value>,
  {
    self
      .properties
      .extend(iter.into_iter().map(|(k, v)| (k.into(), v.into())));
    self
  }

  pub fn build(self) -> Result<Method> {
    let id: DID = self.id.ok_or(Error::InvalidBuilder {
      name: "Method",
      error: "Missing `id`",
    })?;

    let controller: DID = self.controller.ok_or(Error::InvalidBuilder {
      name: "Method",
      error: "Missing `controller`",
    })?;

    let key_type: MethodType = self.key_type.ok_or(Error::InvalidBuilder {
      name: "Method",
      error: "Missing `key_type`",
    })?;

    let key_data: MethodData = self.key_data.ok_or(Error::InvalidBuilder {
      name: "Method",
      error: "Missing `key_data`",
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

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  #[should_panic = "Missing `id`"]
  fn test_missing_id() {
    MethodBuilder::new()
      .controller("did:example:123".parse().unwrap())
      .key_type(MethodType::Ed25519VerificationKey2018)
      .key_data(MethodData::PublicKeyBase58("".into()))
      .build()
      .unwrap();
  }

  #[test]
  #[should_panic = "Missing `key_type`"]
  fn test_missing_key_type() {
    MethodBuilder::new()
      .id("did:example:123".parse().unwrap())
      .controller("did:example:123".parse().unwrap())
      .key_data(MethodData::PublicKeyBase58("".into()))
      .build()
      .unwrap();
  }

  #[test]
  #[should_panic = "Missing `key_data`"]
  fn test_missing_key_data() {
    MethodBuilder::new()
      .id("did:example:123".parse().unwrap())
      .controller("did:example:123".parse().unwrap())
      .key_type(MethodType::Ed25519VerificationKey2018)
      .build()
      .unwrap();
  }

  #[test]
  #[should_panic = "Missing `controller`"]
  fn test_missing_controller() {
    MethodBuilder::new()
      .id("did:example:123".parse().unwrap())
      .key_type(MethodType::Ed25519VerificationKey2018)
      .key_data(MethodData::PublicKeyBase58("".into()))
      .build()
      .unwrap();
  }
}
