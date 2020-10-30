#![feature(box_syntax, const_float_classify)]
#![no_std]

#[cfg(not(feature = "alloc"))]
compile_error!("This crate does not yet support environments without liballoc.");

#[cfg(feature = "alloc")]
extern crate alloc;
#[cfg(feature = "std")]
extern crate std;

#[macro_use]
extern crate serde;

pub extern crate url;

mod document;
mod error;
mod service;
mod utils;
mod verification;

pub use self::document::Document;
pub use self::document::DocumentBuilder;

pub use self::error::Error;
pub use self::error::Result;

pub use self::service::Service;
pub use self::service::ServiceBuilder;

pub use self::utils::Object;
pub use self::utils::OrderedSet;
pub use self::utils::Value;

pub use self::verification::Method;
pub use self::verification::MethodBuilder;
pub use self::verification::MethodData;
pub use self::verification::MethodIndex;
pub use self::verification::MethodRef;
pub use self::verification::MethodScope;
pub use self::verification::MethodType;
pub use self::verification::MethodWrap;
