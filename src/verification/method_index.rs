use did_url::DID;

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum MethodIndex<'a> {
  Index(usize),
  Ident(&'a str),
}

impl<'a> MethodIndex<'a> {
  pub fn matches(&self, did: &DID) -> bool {
    match self {
      Self::Index(_) => false,
      Self::Ident(ident) if ident.starts_with('#') => {
        matches!(did.fragment(), Some(fragment) if fragment == &ident[1..])
      }
      Self::Ident(ident) if ident.starts_with(DID::SCHEME) && !ident.ends_with('#') => {
        if let Some(index) = ident.rfind('#') {
          matches!(did.fragment(), Some(fragment) if fragment == &ident[index + 1..])
        } else {
          false
        }
      }
      Self::Ident(ident) => matches!(did.fragment(), Some(fragment) if fragment == *ident),
    }
  }
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
