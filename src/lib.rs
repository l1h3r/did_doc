#![no_std]

#[cfg(not(feature = "alloc"))]
compile_error!("This crate does not yet support environments without liballoc.");

#[cfg(feature = "alloc")]
extern crate alloc;
#[cfg(feature = "std")]
extern crate std;

#[macro_use]
extern crate serde;

// Re-export `did_url` for convenience.
pub use did_url;

// Re-export `url` for convenience.
pub use url;

mod document;
mod error;
mod service;
mod utils;
mod verifiable;
mod verification;

pub use self::document::Document;
pub use self::document::DocumentBuilder;

pub use self::error::Error;
pub use self::error::Result;

pub use self::service::Service;
pub use self::service::ServiceBuilder;

pub use self::utils::DIDKey;
pub use self::utils::Object;
pub use self::utils::OrderedSet;
pub use self::utils::Value;

pub use self::verifiable::DocumentReader;
pub use self::verifiable::DocumentWriter;
pub use self::verifiable::MethodReader;
pub use self::verifiable::MethodWriter;
pub use self::verifiable::SetSignature;
pub use self::verifiable::Signature;
pub use self::verifiable::SignatureData;
pub use self::verifiable::SignatureDocument;
pub use self::verifiable::SignatureOptions;
pub use self::verifiable::SignatureSuite;
pub use self::verifiable::SignatureValue;
pub use self::verifiable::TrySignature;
pub use self::verifiable::TrySignatureMut;
pub use self::verifiable::VerifiableDocument;
pub use self::verifiable::VerifiableProperties;

pub use self::verification::Method;
pub use self::verification::MethodBuilder;
pub use self::verification::MethodData;
pub use self::verification::MethodIndex;
pub use self::verification::MethodQuery;
pub use self::verification::MethodRef;
pub use self::verification::MethodScope;
pub use self::verification::MethodType;
pub use self::verification::MethodWrap;

mod lib {
  #[cfg(all(feature = "alloc", not(feature = "std")))]
  pub use alloc::borrow::Borrow;
  #[cfg(feature = "std")]
  pub use std::borrow::Borrow;

  #[cfg(all(feature = "alloc", not(feature = "std")))]
  pub use alloc::string::{String, ToString};
  #[cfg(feature = "std")]
  pub use std::string::{String, ToString};

  #[cfg(all(feature = "alloc", not(feature = "std")))]
  pub use alloc::vec::Vec;
  #[cfg(feature = "std")]
  pub use std::vec::Vec;

  #[cfg(all(feature = "alloc", not(feature = "std")))]
  pub use alloc::collections::BTreeMap;
  #[cfg(feature = "std")]
  pub use std::collections::BTreeMap;

}
