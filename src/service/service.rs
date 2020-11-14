use alloc::string::String;
use core::fmt::Display;
use core::fmt::Error as FmtError;
use core::fmt::Formatter;
use core::fmt::Result as FmtResult;
use did_url::DID;
use serde::Serialize;
use serde_json::to_string;
use serde_json::to_string_pretty;
use url::Url;

use crate::error::Error;
use crate::error::Result;
use crate::service::ServiceBuilder;
use crate::utils::Object;

const ERR_MI: &str = "Missing `id`";
const ERR_MT: &str = "Missing `type`";
const ERR_MS: &str = "Missing `service_endpoint`";

/// A DID Document Service
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct Service<T = Object> {
  pub(crate) id: DID,
  #[serde(rename = "type")]
  pub(crate) type_: String,
  #[serde(rename = "serviceEndpoint")]
  pub(crate) service_endpoint: Url,
  #[serde(flatten)]
  pub(crate) properties: T,
}

impl<T> Service<T> {
  /// Creates a `ServiceBuilder` to configure a new `Service`.
  ///
  /// This is the same as `ServiceBuilder::new()`.
  pub fn builder(properties: T) -> ServiceBuilder<T> {
    ServiceBuilder::new(properties)
  }

  /// Returns a new `Service` based on the `ServiceBuilder` configuration.
  pub fn from_builder(builder: ServiceBuilder<T>) -> Result<Self> {
    let id: DID = builder.id.ok_or(Error::InvalidBuilder {
      name: "Service",
      error: ERR_MI,
    })?;

    let type_: String = builder.type_.ok_or(Error::InvalidBuilder {
      name: "Service",
      error: ERR_MT,
    })?;

    let service_endpoint: Url = builder.service_endpoint.ok_or(Error::InvalidBuilder {
      name: "Service",
      error: ERR_MS,
    })?;

    Ok(Self {
      id,
      type_,
      service_endpoint,
      properties: builder.properties,
    })
  }

  /// Returns a reference to the `Service` id.
  pub fn id(&self) -> &DID {
    &self.id
  }

  /// Returns a mutable reference to the `Service` id.
  pub fn id_mut(&mut self) -> &mut DID {
    &mut self.id
  }

  /// Returns a reference to the `Service` type.
  pub fn type_(&self) -> &str {
    &*self.type_
  }

  /// Returns a mutable reference to the `Service` type.
  pub fn type_mut(&mut self) -> &mut String {
    &mut self.type_
  }

  /// Returns a reference to the `Service` endpoint.
  pub fn service_endpoint(&self) -> &Url {
    &self.service_endpoint
  }

  /// Returns a mutable reference to the `Service` endpoint.
  pub fn service_endpoint_mut(&mut self) -> &mut Url {
    &mut self.service_endpoint
  }

  /// Returns a reference to the custom `Service` properties.
  pub fn properties(&self) -> &T {
    &self.properties
  }

  /// Returns a mutable reference to the custom `Service` properties.
  pub fn properties_mut(&mut self) -> &mut T {
    &mut self.properties
  }
}

impl<T> Display for Service<T>
where
  T: Serialize,
{
  fn fmt(&self, f: &mut Formatter) -> FmtResult {
    if f.alternate() {
      f.write_str(&to_string_pretty(self).map_err(|_| FmtError)?)
    } else {
      f.write_str(&to_string(self).map_err(|_| FmtError)?)
    }
  }
}

impl<T> AsRef<DID> for Service<T> {
  fn as_ref(&self) -> &DID {
    self.id()
  }
}
