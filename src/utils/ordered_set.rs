use alloc::collections::vec_deque::Iter;
use alloc::collections::VecDeque;
use alloc::vec::Vec;
use core::convert::TryFrom;
use core::fmt::Debug;
use core::fmt::Formatter;
use core::fmt::Result as FmtResult;
use core::ops::Deref;

use crate::error::Error;
use crate::error::Result;

#[derive(Clone, Hash, PartialEq, Eq, PartialOrd, Ord, Deserialize, Serialize)]
#[repr(transparent)]
#[serde(transparent)]
pub struct OrderedSet<T>(VecDeque<T>);

impl<T> OrderedSet<T> {
  pub fn new() -> Self {
    Self(VecDeque::new())
  }

  pub fn with_capacity(capacity: usize) -> Self {
    Self(VecDeque::with_capacity(capacity))
  }

  pub fn len(&self) -> usize {
    self.0.len()
  }

  pub fn is_empty(&self) -> bool {
    self.0.is_empty()
  }

  pub fn iter(&self) -> Iter<T> {
    self.0.iter()
  }

  pub fn head(&self) -> Option<&T> {
    self.0.front()
  }

  pub fn tail(&self) -> Option<&T> {
    self.0.back()
  }

  pub fn as_slice(&self) -> &[T] {
    self.0.as_slices().0
  }

  pub fn into_vec(self) -> Vec<T> {
    self.0.into_iter().collect()
  }

  pub fn append(&mut self, item: T) -> bool
  where
    T: PartialEq,
  {
    if self.0.contains(&item) {
      false
    } else {
      self.0.push_back(item);
      self.0.make_contiguous();
      true
    }
  }

  pub fn prepend(&mut self, item: T) -> bool
  where
    T: PartialEq,
  {
    if self.0.contains(&item) {
      false
    } else {
      self.0.push_front(item);
      self.0.make_contiguous();
      true
    }
  }

  pub fn replace(&mut self, current: &T, replacement: T)
  where
    T: PartialEq,
  {
    let index: Option<usize> = self
      .0
      .iter()
      .position(|item| item == current || *item == replacement);

    if let Some(index) = index {
      let tail: Vec<T> = self.0.drain(index..).collect();
      let iter: _ = tail
        .into_iter()
        .filter(|item| item != current && *item != replacement);

      self.0.extend(iter);
      self.0.insert(index, replacement);
      self.0.make_contiguous();
    }
  }
}

impl<T> Debug for OrderedSet<T>
where
  T: Debug,
{
  fn fmt(&self, f: &mut Formatter) -> FmtResult {
    f.debug_set().entries(self.iter()).finish()
  }
}

impl<T> Deref for OrderedSet<T> {
  type Target = [T];

  fn deref(&self) -> &Self::Target {
    self.as_slice()
  }
}

impl<T> Default for OrderedSet<T> {
  fn default() -> Self {
    Self::new()
  }
}

impl<T> TryFrom<Vec<T>> for OrderedSet<T>
where
  T: PartialEq,
{
  type Error = Error;

  fn try_from(other: Vec<T>) -> Result<Self, Self::Error> {
    let mut this: Self = Self::with_capacity(other.len());

    for item in other {
      if !this.append(item) {
        return Err(Error::InvalidSet {
          error: "Duplicate Item",
        });
      }
    }

    Ok(this)
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_works() {
    let mut set = OrderedSet::new();

    set.append("a");
    set.append("b");
    set.append("c");

    assert_eq!(set.as_slice(), &["a", "b", "c"]);
    assert_eq!(set.head(), Some(&"a"));
    assert_eq!(set.tail(), Some(&"c"));

    set.replace(&"a", "c");

    assert_eq!(set.as_slice(), &["c", "b"]);

    let mut set = OrderedSet::new();

    set.prepend("a");
    set.prepend("b");
    set.prepend("c");

    assert_eq!(set.as_slice(), &["c", "b", "a"]);
    assert_eq!(set.head(), Some(&"c"));
    assert_eq!(set.tail(), Some(&"a"));

    set.replace(&"a", "c");

    assert_eq!(set.as_slice(), &["c", "b"]);
  }
}
