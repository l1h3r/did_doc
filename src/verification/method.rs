use did::DID;

use crate::utils::Object;
use crate::verification::MethodData;
use crate::verification::MethodType;

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct Method {
  pub(crate) id: DID,
  pub(crate) controller: DID,
  #[serde(rename = "type")]
  pub(crate) key_type: MethodType,
  #[serde(flatten)]
  pub(crate) key_data: MethodData,
  #[serde(flatten)]
  pub(crate) properties: Object,
}

impl Method {
  pub fn id(&self) -> &DID {
    &self.id
  }

  pub fn controller(&self) -> &DID {
    &self.controller
  }

  pub fn key_type(&self) -> MethodType {
    self.key_type
  }

  pub fn key_data(&self) -> &MethodData {
    &self.key_data
  }

  pub fn properties(&self) -> &Object {
    &self.properties
  }

  pub fn properties_mut(&mut self) -> &mut Object {
    &mut self.properties
  }
}
