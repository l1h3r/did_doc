use alloc::string::String;
use did_url::DID;
use url::Url;

use crate::error::Error;
use crate::error::Result;
use crate::service::Service;
use crate::utils::Object;
use crate::utils::Value;

#[derive(Clone, Debug, Default)]
pub struct ServiceBuilder {
  id: Option<DID>,
  type_: Option<String>,
  service_endpoint: Option<Url>,
  properties: Object,
}

impl ServiceBuilder {
  pub fn new() -> Self {
    Self {
      id: None,
      type_: None,
      service_endpoint: None,
      properties: Object::new(),
    }
  }

  pub fn id(mut self, value: DID) -> Self {
    self.id = Some(value);
    self
  }

  pub fn type_(mut self, value: impl Into<String>) -> Self {
    self.type_ = Some(value.into());
    self
  }

  pub fn service_endpoint(mut self, value: Url) -> Self {
    self.service_endpoint = Some(value);
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

  pub fn build(self) -> Result<Service> {
    let id: DID = self.id.ok_or(Error::InvalidBuilder {
      name: "Service",
      error: "Missing `id`",
    })?;

    let type_: String = self.type_.ok_or(Error::InvalidBuilder {
      name: "Service",
      error: "Missing `type`",
    })?;

    let service_endpoint: Url = self.service_endpoint.ok_or(Error::InvalidBuilder {
      name: "Service",
      error: "Missing `service_endpoint`",
    })?;

    Ok(Service {
      id,
      type_,
      service_endpoint,
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
    ServiceBuilder::new()
      .type_("ServiceType")
      .service_endpoint("https://example.com".parse().unwrap())
      .build()
      .unwrap();
  }

  #[test]
  #[should_panic = "Missing `type`"]
  fn test_missing_type_() {
    ServiceBuilder::new()
      .id("did:example:123".parse().unwrap())
      .service_endpoint("https://example.com".parse().unwrap())
      .build()
      .unwrap();
  }

  #[test]
  #[should_panic = "Missing `service_endpoint`"]
  fn test_missing_service_endpoint() {
    ServiceBuilder::new()
      .id("did:example:123".parse().unwrap())
      .type_("ServiceType")
      .build()
      .unwrap();
  }
}
