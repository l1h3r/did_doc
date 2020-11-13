use alloc::string::String;
use did_url::DID;
use url::Url;

use crate::error::Error;
use crate::error::Result;
use crate::service::Service;
use crate::utils::Object;
use crate::utils::Value;

const ERR_MI: &str = "Missing `id`";
const ERR_MT: &str = "Missing `type`";
const ERR_MS: &str = "Missing `service_endpoint`";

#[derive(Clone, Debug, Default)]
pub struct ServiceBuilder<T = Object> {
  id: Option<DID>,
  type_: Option<String>,
  service_endpoint: Option<Url>,
  properties: T,
}

impl<T> ServiceBuilder<T> {
  #[must_use]
  pub fn new(properties: T) -> Self {
    Self {
      id: None,
      type_: None,
      service_endpoint: None,
      properties,
    }
  }

  #[must_use]
  pub fn id(mut self, value: DID) -> Self {
    self.id = Some(value);
    self
  }

  #[must_use]
  pub fn type_(mut self, value: impl Into<String>) -> Self {
    self.type_ = Some(value.into());
    self
  }

  #[must_use]
  pub fn service_endpoint(mut self, value: Url) -> Self {
    self.service_endpoint = Some(value);
    self
  }

  #[must_use]
  pub fn build(self) -> Result<Service<T>> {
    let id: DID = self.id.ok_or(Error::InvalidBuilder {
      name: "Service",
      error: ERR_MI,
    })?;

    let type_: String = self.type_.ok_or(Error::InvalidBuilder {
      name: "Service",
      error: ERR_MT,
    })?;

    let service_endpoint: Url = self.service_endpoint.ok_or(Error::InvalidBuilder {
      name: "Service",
      error: ERR_MS,
    })?;

    Ok(Service {
      id,
      type_,
      service_endpoint,
      properties: self.properties,
    })
  }
}

impl ServiceBuilder {
  #[must_use]
  pub fn property<K, V>(mut self, key: K, value: V) -> Self
  where
    K: Into<String>,
    V: Into<Value>,
  {
    self.properties.insert(key.into(), value.into());
    self
  }

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
    let _: Service = ServiceBuilder::default()
      .type_("ServiceType")
      .service_endpoint("https://example.com".parse().unwrap())
      .build()
      .unwrap();
  }

  #[test]
  #[should_panic = "Missing `type`"]
  fn test_missing_type_() {
    let _: Service = ServiceBuilder::default()
      .id("did:example:123".parse().unwrap())
      .service_endpoint("https://example.com".parse().unwrap())
      .build()
      .unwrap();
  }

  #[test]
  #[should_panic = "Missing `service_endpoint`"]
  fn test_missing_service_endpoint() {
    let _: Service = ServiceBuilder::default()
      .id("did:example:123".parse().unwrap())
      .type_("ServiceType")
      .build()
      .unwrap();
  }
}
