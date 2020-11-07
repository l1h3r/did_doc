use core::cell::Cell;
use core::ops::Deref;
use core::ops::DerefMut;

use crate::verifiable::SignatureData;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Deserialize, Serialize)]
#[serde(transparent)]
pub struct SignatureValue {
  data: SignatureData,
  #[serde(skip)]
  hide: Cell<bool>,
}

impl SignatureValue {
  pub fn new() -> Self {
    Self {
      data: SignatureData::None,
      hide: Cell::new(false),
    }
  }

  pub fn is_none(&self) -> bool {
    self.data.is_none() || self.hide.get()
  }

  pub fn set(&mut self, value: SignatureData) {
    self.data = value;
  }

  pub(crate) fn hide(&self) {
    self.hide.set(true);
  }

  pub(crate) fn show(&self) {
    self.hide.set(false);
  }
}

impl Default for SignatureValue {
  fn default() -> Self {
    Self::new()
  }
}

impl Deref for SignatureValue {
  type Target = SignatureData;

  fn deref(&self) -> &Self::Target {
    &self.data
  }
}

impl DerefMut for SignatureValue {
  fn deref_mut(&mut self) -> &mut Self::Target {
    &mut self.data
  }
}
