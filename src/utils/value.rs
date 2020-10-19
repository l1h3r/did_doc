use alloc::borrow::Cow;
use alloc::string::String;
use alloc::string::ToString as _;
use alloc::vec::Vec;
use core::cmp::Ordering;
use core::convert::TryFrom;
use core::fmt::Display;
use core::fmt::Formatter;
use core::fmt::Result as FmtResult;
use core::hash::Hash;
use core::hash::Hasher;
use core::iter::FromIterator;
use core::num::FpCategory;

use crate::utils::Object;

// =============================================================================
// Value
// =============================================================================

macro_rules! impl_primitive_value {
  ($($ty:ty),*) => {
    $(
      impl From<$ty> for Value {
        fn from(other: $ty) -> Self {
          Self::Number(other.into())
        }
      }
    )*
  };
}

#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Deserialize, Serialize)]
#[serde(untagged)]
pub enum Value {
  Null,
  Bool(bool),
  Number(Number),
  String(String),
  Array(Vec<Self>),
  Object(Object),
}

impl From<()> for Value {
  fn from(_: ()) -> Self {
    Self::Null
  }
}

impl From<bool> for Value {
  fn from(value: bool) -> Self {
    Self::Bool(value)
  }
}

impl From<Number> for Value {
  fn from(value: Number) -> Self {
    Self::Number(value)
  }
}

impl_primitive_value! {
  u8, u16, u32, u64,
  i8, i16, i32, i64
}

impl From<f32> for Value {
  fn from(other: f32) -> Self {
    Number::try_from(other).map_or(Self::Null, Self::Number)
  }
}

impl From<f64> for Value {
  fn from(other: f64) -> Self {
    Number::try_from(other).map_or(Self::Null, Self::Number)
  }
}

impl From<String> for Value {
  fn from(other: String) -> Self {
    Self::String(other)
  }
}

impl<'a> From<&'a str> for Value {
  fn from(other: &str) -> Self {
    Self::String(other.to_string())
  }
}

impl<'a> From<Cow<'a, str>> for Value {
  fn from(other: Cow<'a, str>) -> Self {
    Self::String(other.into_owned())
  }
}

impl From<Object> for Value {
  fn from(other: Object) -> Self {
    Self::Object(other)
  }
}

impl<T> From<Vec<T>> for Value
where
  T: Into<Self>,
{
  fn from(other: Vec<T>) -> Self {
    Self::Array(other.into_iter().map(Into::into).collect())
  }
}

impl<'a, T> From<&'a [T]> for Value
where
  T: Clone + Into<Self>,
{
  fn from(other: &'a [T]) -> Self {
    Self::Array(other.iter().cloned().map(Into::into).collect())
  }
}

impl<T> FromIterator<T> for Value
where
  T: Into<Self>,
{
  fn from_iter<I>(iter: I) -> Self
  where
    I: IntoIterator<Item = T>,
  {
    Self::Array(iter.into_iter().map(Into::into).collect())
  }
}

// =============================================================================
// Number
// =============================================================================

macro_rules! impl_primitive_number {
  ($($ident:ident($($ty:ty),*)),*) => {
    $(
      $(
        impl From<$ty> for Number {
          #[inline(always)]
          fn from(other: $ty) -> Self {
            Self::$ident(other.into())
          }
        }
      )*
    )*
  };
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Deserialize, Serialize)]
#[serde(untagged)]
pub enum Number {
  PosInt(u64),
  NegInt(i64),
  Float(Float),
}

impl Display for Number {
  fn fmt(&self, f: &mut Formatter) -> FmtResult {
    match self {
      Self::PosInt(inner) => Display::fmt(&inner, f),
      Self::NegInt(inner) => Display::fmt(&inner, f),
      Self::Float(inner) => Display::fmt(&inner, f),
    }
  }
}

impl_primitive_number! {
  PosInt(u8, u16, u32, u64),
  NegInt(i8, i16, i32, i64)
}

impl Default for Number {
  fn default() -> Self {
    Self::PosInt(0)
  }
}

impl From<Float> for Number {
  fn from(other: Float) -> Self {
    Self::Float(other)
  }
}

impl TryFrom<f32> for Number {
  type Error = TryFromFloatError;

  fn try_from(other: f32) -> Result<Self, Self::Error> {
    Float::try_from(other).map(Self::Float)
  }
}

impl TryFrom<f64> for Number {
  type Error = TryFromFloatError;

  fn try_from(other: f64) -> Result<Self, Self::Error> {
    Float::try_from(other).map(Self::Float)
  }
}

// =============================================================================
// Float
// =============================================================================

#[derive(Clone, Copy, Debug, Default, Deserialize, Serialize)]
#[serde(try_from = "f64", into = "f64")]
#[repr(transparent)]
pub struct Float(f64);

impl Float {
  pub const fn try_new(value: f64) -> Option<Self> {
    if Self::is_valid(value) {
      Some(Self(value))
    } else {
      None
    }
  }

  pub const fn as_f64(self) -> f64 {
    self.0
  }

  pub const fn is_valid(value: f64) -> bool {
    !Self::is_invalid(value)
  }

  pub const fn is_invalid(value: f64) -> bool {
    matches!(value.classify(), FpCategory::Nan | FpCategory::Infinite)
  }
}

impl Hash for Float {
  fn hash<H>(&self, hasher: &mut H)
  where
    H: Hasher,
  {
    self.0.to_bits().hash(hasher)
  }
}

impl PartialEq for Float {
  fn eq(&self, other: &Self) -> bool {
    self.0.to_bits().eq(&other.0.to_bits())
  }
}

impl Eq for Float {}

impl PartialOrd for Float {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    self.0.to_bits().partial_cmp(&other.0.to_bits())
  }
}

impl Ord for Float {
  fn cmp(&self, other: &Self) -> Ordering {
    self.partial_cmp(other).unwrap()
  }
}

impl Display for Float {
  fn fmt(&self, f: &mut Formatter) -> FmtResult {
    Display::fmt(&self.0, f)
  }
}

impl From<Float> for f64 {
  fn from(other: Float) -> Self {
    other.as_f64()
  }
}

impl TryFrom<f32> for Float {
  type Error = TryFromFloatError;

  fn try_from(other: f32) -> Result<Self, Self::Error> {
    Self::try_from(other as f64)
  }
}

impl TryFrom<f64> for Float {
  type Error = TryFromFloatError;

  fn try_from(other: f64) -> Result<Self, Self::Error> {
    Self::try_new(other).ok_or(TryFromFloatError(()))
  }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TryFromFloatError(());

impl Display for TryFromFloatError {
  fn fmt(&self, f: &mut Formatter) -> FmtResult {
    f.write_str("invalid float conversion attempted")
  }
}

#[cfg(feature = "std")]
impl std::error::Error for TryFromFloatError {}
