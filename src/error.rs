use core::fmt::Debug;
use core::fmt::Display;
use core::fmt::Formatter;
use core::fmt::Result as FmtResult;

pub type Result<T, E = Error> = core::result::Result<T, E>;

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum Error {
  InvalidBuilder {
    name: &'static str,
    error: &'static str,
  },
  InvalidSet {
    error: &'static str,
  },
  InvalidKey {
    error: &'static str,
  },
  InvalidDID {
    error: did_url::Error,
  },
}

impl Display for Error {
  fn fmt(&self, f: &mut Formatter) -> FmtResult {
    match self {
      Self::InvalidBuilder { name, error } => {
        f.write_fmt(format_args!("Invalid Builder({}): {}", name, error))
      }
      Self::InvalidSet { error } => f.write_fmt(format_args!("Invalid Set: {}", error)),
      Self::InvalidKey { error } => f.write_fmt(format_args!("Invalid Key: {}", error)),
      Self::InvalidDID { error } => Display::fmt(error, f),
    }
  }
}

impl From<did_url::Error> for Error {
  fn from(other: did_url::Error) -> Self {
    Self::InvalidDID { error: other }
  }
}

#[cfg(feature = "std")]
impl ::std::error::Error for Error {}
