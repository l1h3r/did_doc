use core::cmp::Ordering;
use core::convert::AsMut;
use core::convert::AsRef;
use core::ops::Deref;
use core::ops::DerefMut;
use did_url::DID;

#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
#[repr(transparent)]
#[serde(transparent)]
pub struct DIDKey<T>(T);

impl<T> DIDKey<T> {
  #[inline]
  pub const fn new(inner: T) -> Self {
    Self(inner)
  }

  #[inline]
  pub fn into_inner(self) -> T {
    self.0
  }

  #[inline]
  pub fn as_did(&self) -> &DID
  where
    T: AsRef<DID>,
  {
    self.0.as_ref()
  }
}

impl<T> PartialEq for DIDKey<T>
where
  T: AsRef<DID>,
{
  #[inline]
  fn eq(&self, other: &Self) -> bool {
    self.as_did().eq(other.as_did())
  }
}

impl<T> Eq for DIDKey<T> where T: AsRef<DID> {}

impl<T> PartialOrd for DIDKey<T>
where
  T: AsRef<DID>,
{
  #[inline]
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    self.as_did().partial_cmp(other.as_did())
  }
}

impl<T> Ord for DIDKey<T>
where
  T: AsRef<DID>,
{
  #[inline]
  fn cmp(&self, other: &Self) -> Ordering {
    self.as_did().cmp(other.as_did())
  }
}

impl<T> Deref for DIDKey<T> {
  type Target = T;

  #[inline]
  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

impl<T> DerefMut for DIDKey<T> {
  #[inline]
  fn deref_mut(&mut self) -> &mut Self::Target {
    &mut self.0
  }
}

impl<T> AsRef<T> for DIDKey<T> {
  #[inline]
  fn as_ref(&self) -> &T {
    &self.0
  }
}

impl<T> AsMut<T> for DIDKey<T> {
  #[inline]
  fn as_mut(&mut self) -> &mut T {
    &mut self.0
  }
}

impl<T> From<T> for DIDKey<T> {
  #[inline]
  fn from(other: T) -> Self {
    Self(other)
  }
}
