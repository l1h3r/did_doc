use alloc::string::String;
use did::DID;
use url::Url;

use crate::utils::Object;

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct Service {
  pub(crate) id: DID,
  #[serde(rename = "type")]
  pub(crate) type_: String,
  #[serde(rename = "serviceEndpoint")]
  pub(crate) service_endpoint: Url,
  #[serde(flatten)]
  pub(crate) properties: Object,
}

impl Service {
  pub fn id(&self) -> &DID {
    &self.id
  }

  pub fn type_(&self) -> &str {
    &*self.type_
  }

  pub fn service_endpoint(&self) -> &Url {
    &self.service_endpoint
  }

  pub fn properties(&self) -> &Object {
    &self.properties
  }

  pub fn properties_mut(&mut self) -> &mut Object {
    &mut self.properties
  }
}
