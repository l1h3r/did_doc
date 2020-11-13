use alloc::string::String;
use alloc::vec::Vec;
use core::convert::TryInto as _;
use did_url::DID;
use url::Url;

use crate::document::Document;
use crate::error::Error;
use crate::error::Result;
use crate::service::Service;
use crate::utils::DIDKey;
use crate::utils::Object;
use crate::utils::Value;
use crate::verification::Method;
use crate::verification::MethodRef;

const ERR_MI: &str = "Missing `id`";

#[derive(Clone, Debug, Default)]
pub struct DocumentBuilder<T = Object, U = Object, V = Object> {
  id: Option<DID>,
  controller: Option<DID>,
  also_known_as: Vec<Url>,
  verification_method: Vec<DIDKey<Method<U>>>,
  authentication: Vec<DIDKey<MethodRef<U>>>,
  assertion_method: Vec<DIDKey<MethodRef<U>>>,
  key_agreement: Vec<DIDKey<MethodRef<U>>>,
  capability_delegation: Vec<DIDKey<MethodRef<U>>>,
  capability_invocation: Vec<DIDKey<MethodRef<U>>>,
  service: Vec<Service<V>>,
  properties: T,
}

impl<T, U, V> DocumentBuilder<T, U, V> {
  pub fn new(properties: T) -> Self {
    Self {
      id: None,
      controller: None,
      also_known_as: Vec::new(),
      verification_method: Vec::new(),
      authentication: Vec::new(),
      assertion_method: Vec::new(),
      key_agreement: Vec::new(),
      capability_delegation: Vec::new(),
      capability_invocation: Vec::new(),
      service: Vec::new(),
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

  pub fn also_known_as(mut self, value: Url) -> Self {
    self.also_known_as.push(value);
    self
  }

  pub fn verification_method(mut self, value: Method<U>) -> Self {
    self.verification_method.push(DIDKey::new(value));
    self
  }

  pub fn authentication(mut self, value: impl Into<MethodRef<U>>) -> Self {
    self.authentication.push(DIDKey::new(value.into()));
    self
  }

  pub fn assertion_method(mut self, value: impl Into<MethodRef<U>>) -> Self {
    self.assertion_method.push(DIDKey::new(value.into()));
    self
  }

  pub fn key_agreement(mut self, value: impl Into<MethodRef<U>>) -> Self {
    self.key_agreement.push(DIDKey::new(value.into()));
    self
  }

  pub fn capability_delegation(mut self, value: impl Into<MethodRef<U>>) -> Self {
    self.capability_delegation.push(DIDKey::new(value.into()));
    self
  }

  pub fn capability_invocation(mut self, value: impl Into<MethodRef<U>>) -> Self {
    self.capability_invocation.push(DIDKey::new(value.into()));
    self
  }

  pub fn service(mut self, value: Service<V>) {
    self.service.push(value);
  }

  pub fn build(self) -> Result<Document<T, U, V>> {
    let id: DID = self.id.ok_or(Error::InvalidBuilder {
      name: "Document",
      error: ERR_MI,
    })?;

    // TODO: Validate key identifiers

    Ok(Document {
      id,
      controller: self.controller,
      also_known_as: self.also_known_as,
      verification_method: self.verification_method.try_into()?,
      authentication: self.authentication.try_into()?,
      assertion_method: self.assertion_method.try_into()?,
      key_agreement: self.key_agreement.try_into()?,
      capability_delegation: self.capability_delegation.try_into()?,
      capability_invocation: self.capability_invocation.try_into()?,
      service: self.service, // TODO: UnorderedSet
      properties: self.properties,
    })
  }
}

impl DocumentBuilder {
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
    let _: Document = DocumentBuilder::default().build().unwrap();
  }
}
