use alloc::string::String;
use core::fmt::Display;
use core::fmt::Error;
use core::fmt::Formatter;
use core::fmt::Result;
use did_url::DID;
use serde::Serialize;
use serde_json::to_string;
use serde_json::to_string_pretty;
use url::Url;

use crate::utils::Object;

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
  pub fn id(&self) -> &DID {
    &self.id
  }

  pub fn id_mut(&mut self) -> &mut DID {
    &mut self.id
  }

  pub fn type_(&self) -> &str {
    &*self.type_
  }

  pub fn type_mut(&mut self) -> &mut String {
    &mut self.type_
  }

  pub fn service_endpoint(&self) -> &Url {
    &self.service_endpoint
  }

  pub fn service_endpoint_mut(&mut self) -> &mut Url {
    &mut self.service_endpoint
  }

  pub fn properties(&self) -> &T {
    &self.properties
  }

  pub fn properties_mut(&mut self) -> &mut T {
    &mut self.properties
  }
}

impl<T> Display for Service<T>
where
  T: Serialize,
{
  fn fmt(&self, f: &mut Formatter) -> Result {
    if f.alternate() {
      f.write_str(&to_string_pretty(self).map_err(|_| Error)?)
    } else {
      f.write_str(&to_string(self).map_err(|_| Error)?)
    }
  }
}

impl<T> AsRef<DID> for Service<T> {
  fn as_ref(&self) -> &DID {
    self.id()
  }
}
