use core::fmt::Display;
use core::fmt::Error;
use core::fmt::Formatter;
use core::fmt::Result;
use did_url::DID;
use serde::Serialize;
use serde_json::to_string;
use serde_json::to_string_pretty;

use crate::utils::Object;
use crate::verification::MethodData;
use crate::verification::MethodType;

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct Method<T = Object> {
  pub(crate) id: DID,
  pub(crate) controller: DID,
  #[serde(rename = "type")]
  pub(crate) key_type: MethodType,
  #[serde(flatten)]
  pub(crate) key_data: MethodData,
  #[serde(flatten)]
  pub(crate) properties: T,
}

impl<T> Method<T> {
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

  pub fn properties(&self) -> &T {
    &self.properties
  }

  pub fn properties_mut(&mut self) -> &mut T {
    &mut self.properties
  }
}

impl<T> Display for Method<T>
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
