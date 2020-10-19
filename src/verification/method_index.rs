#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum MethodIndex<'a> {
  Index(usize),
  Ident(&'a str),
}

impl<'a> From<&'a str> for MethodIndex<'a> {
  fn from(other: &'a str) -> Self {
    Self::Ident(other)
  }
}

impl From<usize> for MethodIndex<'_> {
  fn from(other: usize) -> Self {
    Self::Index(other)
  }
}

impl PartialEq<usize> for MethodIndex<'_> {
  fn eq(&self, other: &usize) -> bool {
    matches!(self, Self::Index(index) if index == other)
  }
}

impl PartialEq<&'_ str> for MethodIndex<'_> {
  fn eq(&self, other: &&'_ str) -> bool {
    matches!(self, Self::Ident(ident) if ident == other)
  }
}
