use core::fmt::Display;
use core::fmt::Error as FmtError;
use core::fmt::Formatter;
use core::fmt::Result as FmtResult;
use core::iter::once;
use did_url::DID;
use serde::Serialize;
use serde_json::to_string;
use serde_json::to_string_pretty;

use crate::lib::*;
use crate::error::Error;
use crate::error::Result;
use crate::utils::Object;
use crate::verification::MethodBuilder;
use crate::verification::MethodData;
use crate::verification::MethodType;

const ERR_MI: &str = "Missing `id`";
const ERR_MC: &str = "Missing `controller`";
const ERR_MKT: &str = "Missing `key_type`";
const ERR_MKD: &str = "Missing `key_data`";
const ERR_VMMF: &str = "Verification Method Missing Fragment";

/// A DID Document Verification Method
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
  /// Creates a `MethodBuilder` to configure a new `Method`.
  ///
  /// This is the same as `MethodBuilder::new()`.
  pub fn builder(properties: T) -> MethodBuilder<T> {
    MethodBuilder::new(properties)
  }

  /// Returns a new `Method` based on the `MethodBuilder` configuration.
  pub fn from_builder(builder: MethodBuilder<T>) -> Result<Self> {
    let id: DID = builder.id.ok_or(Error::InvalidBuilder {
      name: "Method",
      error: ERR_MI,
    })?;

    let controller: DID = builder.controller.ok_or(Error::InvalidBuilder {
      name: "Method",
      error: ERR_MC,
    })?;

    let key_type: MethodType = builder.key_type.ok_or(Error::InvalidBuilder {
      name: "Method",
      error: ERR_MKT,
    })?;

    let key_data: MethodData = builder.key_data.ok_or(Error::InvalidBuilder {
      name: "Method",
      error: ERR_MKD,
    })?;

    Ok(Method {
      id,
      controller,
      key_type,
      key_data,
      properties: builder.properties,
    })
  }

  /// Returns a reference to the verification `Method` id.
  pub fn id(&self) -> &DID {
    &self.id
  }

  /// Returns a mutable reference to the verification `Method` id.
  pub fn id_mut(&mut self) -> &mut DID {
    &mut self.id
  }

  /// Returns a reference to the verification `Method` controller.
  pub fn controller(&self) -> &DID {
    &self.controller
  }

  /// Returns a mutable reference to the verification `Method` controller.
  pub fn controller_mut(&mut self) -> &mut DID {
    &mut self.controller
  }

  /// Returns a reference to the verification `Method` type.
  pub fn key_type(&self) -> MethodType {
    self.key_type
  }

  /// Returns a mutable reference to the verification `Method` type.
  pub fn key_type_mut(&mut self) -> &mut MethodType {
    &mut self.key_type
  }

  /// Returns a reference to the verification `Method` data.
  pub fn key_data(&self) -> &MethodData {
    &self.key_data
  }

  /// Returns a mutable reference to the verification `Method` data.
  pub fn key_data_mut(&mut self) -> &mut MethodData {
    &mut self.key_data
  }

  /// Returns a reference to the custom verification `Method` properties.
  pub fn properties(&self) -> &T {
    &self.properties
  }

  /// Returns a mutable reference to the custom verification `Method` properties.
  pub fn properties_mut(&mut self) -> &mut T {
    &mut self.properties
  }

  pub fn try_into_fragment(&self) -> Result<String> {
    self
      .id
      .fragment()
      .ok_or_else(|| Error::message(ERR_VMMF))
      .map(|fragment| once('#').chain(fragment.chars()).collect())
  }
}

impl<T> Display for Method<T>
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

impl<T> AsRef<DID> for Method<T> {
  fn as_ref(&self) -> &DID {
    self.id()
  }
}
