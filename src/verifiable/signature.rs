use alloc::string::String;
use core::ops::Deref;
use core::ops::DerefMut;

use crate::verifiable::SignatureOptions;
use crate::verifiable::SignatureValue;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Deserialize, Serialize)]
pub struct Signature {
  #[serde(rename = "type")]
  type_: String,
  #[serde(flatten, skip_serializing_if = "SignatureValue::is_none")]
  data: SignatureValue,
  #[serde(flatten)]
  options: SignatureOptions,
}

impl Signature {
  pub fn new(type_: impl Into<String>, options: SignatureOptions) -> Self {
    Self {
      type_: type_.into(),
      options,
      data: SignatureValue::new(),
    }
  }

  pub fn type_(&self) -> &str {
    &*self.type_
  }

  pub const fn data(&self) -> &SignatureValue {
    &self.data
  }

  pub fn data_mut(&mut self) -> &mut SignatureValue {
    &mut self.data
  }

  pub(crate) fn hide_value(&self) {
    self.data.hide();
  }

  pub(crate) fn show_value(&self) {
    self.data.show();
  }
}

impl Deref for Signature {
  type Target = SignatureOptions;

  fn deref(&self) -> &Self::Target {
    &self.options
  }
}

impl DerefMut for Signature {
  fn deref_mut(&mut self) -> &mut Self::Target {
    &mut self.options
  }
}
