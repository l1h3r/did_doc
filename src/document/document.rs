use alloc::vec::Vec;
use core::convert::TryInto as _;
use core::fmt::Display;
use core::fmt::Error as FmtError;
use core::fmt::Formatter;
use core::fmt::Result as FmtResult;
use did_url::DID;
use serde::Serialize;
use serde_json::to_string;
use serde_json::to_string_pretty;
use url::Url;

use crate::document::DocumentBuilder;
use crate::error::Error;
use crate::error::Result;
use crate::service::Service;
use crate::utils::DIDKey;
use crate::utils::Object;
use crate::utils::OrderedSet;
use crate::verification::Method;
use crate::verification::MethodQuery;
use crate::verification::MethodRef;
use crate::verification::MethodScope;
use crate::verification::MethodWrap;

const ERR_VMNF: &str = "Verification Method Not Found";
const ERR_MI: &str = "Missing `id`";

/// A DID Document Service
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[rustfmt::skip]
pub struct Document<T = Object, U = Object, V = Object> {
  pub(crate) id: DID,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub(crate) controller: Option<DID>,
  #[serde(default = "Default::default", rename = "alsoKnownAs", skip_serializing_if = "Vec::is_empty")]
  pub(crate) also_known_as: Vec<Url>,
  #[serde(default = "Default::default", rename = "verificationMethod", skip_serializing_if = "OrderedSet::is_empty")]
  pub(crate) verification_method: OrderedSet<DIDKey<Method<U>>>,
  #[serde(default = "Default::default", skip_serializing_if = "OrderedSet::is_empty")]
  pub(crate) authentication: OrderedSet<DIDKey<MethodRef<U>>>,
  #[serde(default = "Default::default", rename = "assertionMethod", skip_serializing_if = "OrderedSet::is_empty")]
  pub(crate) assertion_method: OrderedSet<DIDKey<MethodRef<U>>>,
  #[serde(default = "Default::default", rename = "keyAgreement", skip_serializing_if = "OrderedSet::is_empty")]
  pub(crate) key_agreement: OrderedSet<DIDKey<MethodRef<U>>>,
  #[serde(default = "Default::default", rename = "capabilityDelegation", skip_serializing_if = "OrderedSet::is_empty")]
  pub(crate) capability_delegation: OrderedSet<DIDKey<MethodRef<U>>>,
  #[serde(default = "Default::default", rename = "capabilityInvocation", skip_serializing_if = "OrderedSet::is_empty")]
  pub(crate) capability_invocation: OrderedSet<DIDKey<MethodRef<U>>>,
  #[serde(default = "Default::default", skip_serializing_if = "Vec::is_empty")]
  pub(crate) service: Vec<Service<V>>,
  #[serde(flatten)]
  pub(crate) properties: T,
}

impl<T, U, V> Document<T, U, V> {
  /// Creates a `DocumentBuilder` to configure a new `Document`.
  ///
  /// This is the same as `DocumentBuilder::new()`.
  pub fn builder(properties: T) -> DocumentBuilder<T, U, V> {
    DocumentBuilder::new(properties)
  }

  /// Returns a new `Document` based on the `DocumentBuilder` configuration.
  pub fn from_builder(builder: DocumentBuilder<T, U, V>) -> Result<Self> {
    let id: DID = builder.id.ok_or(Error::InvalidBuilder {
      name: "Document",
      error: ERR_MI,
    })?;

    // TODO: Validate key identifiers

    Ok(Self {
      id,
      controller: builder.controller,
      also_known_as: builder.also_known_as,
      verification_method: builder.verification_method.try_into()?,
      authentication: builder.authentication.try_into()?,
      assertion_method: builder.assertion_method.try_into()?,
      key_agreement: builder.key_agreement.try_into()?,
      capability_delegation: builder.capability_delegation.try_into()?,
      capability_invocation: builder.capability_invocation.try_into()?,
      service: builder.service, // TODO: UnorderedSet
      properties: builder.properties,
    })
  }

  /// Returns a reference to the `Document` id.
  pub fn id(&self) -> &DID {
    &self.id
  }

  /// Returns a mutable reference to the `Document` id.
  pub fn id_mut(&mut self) -> &mut DID {
    &mut self.id
  }

  /// Returns a reference to the `Document` controller.
  pub fn controller(&self) -> Option<&DID> {
    self.controller.as_ref()
  }

  /// Returns a mutable reference to the `Document` controller.
  pub fn controller_mut(&mut self) -> Option<&mut DID> {
    self.controller.as_mut()
  }

  /// Returns a reference to the `Document` alsoKnownAs set.
  pub fn also_known_as(&self) -> &[Url] {
    &self.also_known_as
  }

  /// Returns a mutable reference to the `Document` alsoKnownAs set.
  pub fn also_known_as_mut(&mut self) -> &mut Vec<Url> {
    &mut self.also_known_as
  }

  /// Returns a reference to the `Document` verificationMethod set.
  pub fn verification_method(&self) -> &OrderedSet<DIDKey<Method<U>>> {
    &self.verification_method
  }

  /// Returns a mutable reference to the `Document` verificationMethod set.
  pub fn verification_method_mut(&mut self) -> &mut OrderedSet<DIDKey<Method<U>>> {
    &mut self.verification_method
  }

  /// Returns a reference to the `Document` authentication set.
  pub fn authentication(&self) -> &OrderedSet<DIDKey<MethodRef<U>>> {
    &self.authentication
  }

  /// Returns a mutable reference to the `Document` authentication set.
  pub fn authentication_mut(&mut self) -> &mut OrderedSet<DIDKey<MethodRef<U>>> {
    &mut self.authentication
  }

  /// Returns a reference to the `Document` assertionMethod set.
  pub fn assertion_method(&self) -> &OrderedSet<DIDKey<MethodRef<U>>> {
    &self.assertion_method
  }

  /// Returns a mutable reference to the `Document` assertionMethod set.
  pub fn assertion_method_mut(&mut self) -> &mut OrderedSet<DIDKey<MethodRef<U>>> {
    &mut self.assertion_method
  }

  /// Returns a reference to the `Document` keyAgreement set.
  pub fn key_agreement(&self) -> &OrderedSet<DIDKey<MethodRef<U>>> {
    &self.key_agreement
  }

  /// Returns a mutable reference to the `Document` keyAgreement set.
  pub fn key_agreement_mut(&mut self) -> &mut OrderedSet<DIDKey<MethodRef<U>>> {
    &mut self.key_agreement
  }

  /// Returns a reference to the `Document` capabilityDelegation set.
  pub fn capability_delegation(&self) -> &OrderedSet<DIDKey<MethodRef<U>>> {
    &self.capability_delegation
  }

  /// Returns a mutable reference to the `Document` capabilityDelegation set.
  pub fn capability_delegation_mut(&mut self) -> &mut OrderedSet<DIDKey<MethodRef<U>>> {
    &mut self.capability_delegation
  }

  /// Returns a reference to the `Document` capabilityInvocation set.
  pub fn capability_invocation(&self) -> &OrderedSet<DIDKey<MethodRef<U>>> {
    &self.capability_invocation
  }

  /// Returns a mutable reference to the `Document` capabilityInvocation set.
  pub fn capability_invocation_mut(&mut self) -> &mut OrderedSet<DIDKey<MethodRef<U>>> {
    &mut self.capability_invocation
  }

  /// Returns a reference to the `Document` service set.
  pub fn service(&self) -> &[Service<V>] {
    &self.service
  }

  /// Returns a mutable reference to the `Document` service set.
  pub fn service_mut(&mut self) -> &mut Vec<Service<V>> {
    &mut self.service
  }

  /// Returns a reference to the custom `Document` properties.
  pub fn properties(&self) -> &T {
    &self.properties
  }

  /// Returns a mutable reference to the custom `Document` properties.
  pub fn properties_mut(&mut self) -> &mut T {
    &mut self.properties
  }

  /// Maps `Document<T>` to `Document<U>` by applying a function to the custom
  /// properties.
  pub fn map<A, F>(self, f: F) -> Document<A, U, V>
  where
    F: FnOnce(T) -> A,
  {
    Document {
      id: self.id,
      controller: self.controller,
      also_known_as: self.also_known_as,
      verification_method: self.verification_method,
      authentication: self.authentication,
      assertion_method: self.assertion_method,
      key_agreement: self.key_agreement,
      capability_delegation: self.capability_delegation,
      capability_invocation: self.capability_invocation,
      service: self.service,
      properties: f(self.properties),
    }
  }

  /// A fallible version of `Document::map(..)`.
  ///
  /// # Errors
  ///
  /// `try_map` can fail if the provided function fails.
  pub fn try_map<A, F, E>(self, f: F) -> Result<Document<A, U, V>, E>
  where
    F: FnOnce(T) -> Result<A, E>,
  {
    Ok(Document {
      id: self.id,
      controller: self.controller,
      also_known_as: self.also_known_as,
      verification_method: self.verification_method,
      authentication: self.authentication,
      assertion_method: self.assertion_method,
      key_agreement: self.key_agreement,
      capability_delegation: self.capability_delegation,
      capability_invocation: self.capability_invocation,
      service: self.service,
      properties: f(self.properties)?,
    })
  }

  /// Finds and returns the first verification `Method` matching the provided
  ///`MethodQuery`.
  pub fn resolve<'a, Q>(&self, query: Q) -> Option<MethodWrap<U>>
  where
    Q: Into<MethodQuery<'a>>,
  {
    self.resolve_method(query.into())
  }

  /// Finds and returns the first verification `Method` matching the provided
  ///`MethodQuery`.
  ///
  /// # Errors
  ///
  /// Fails if no matching verification `Method` is found.
  pub fn try_resolve<'a, Q>(&self, query: Q) -> Result<MethodWrap<U>>
  where
    Q: Into<MethodQuery<'a>>,
  {
    self.resolve(query).ok_or_else(|| Error::message(ERR_VMNF))
  }

  pub fn resolve_bytes<'a, Q>(&self, query: Q) -> Option<Vec<u8>>
  where
    Q: Into<MethodQuery<'a>>,
  {
    self.resolve(query)?.key_data().try_decode().ok()
  }

  pub fn try_resolve_bytes<'a, Q>(&self, query: Q) -> Result<Vec<u8>>
  where
    Q: Into<MethodQuery<'a>>,
  {
    self.try_resolve(query)?.key_data().try_decode()
  }

  fn resolve_method<'a>(&self, query: MethodQuery<'a>) -> Option<MethodWrap<U>> {
    let iter = match query.scope {
      MethodScope::VerificationMethod => return self.resolve_verification_method(query),
      MethodScope::Authentication => self.authentication.iter(),
      MethodScope::AssertionMethod => self.assertion_method.iter(),
      MethodScope::KeyAgreement => self.key_agreement.iter(),
      MethodScope::CapabilityDelegation => self.capability_delegation.iter(),
      MethodScope::CapabilityInvocation => self.capability_invocation.iter(),
    };

    iter
      .enumerate()
      .find(|(index, method)| query.ident == *index || query.ident.matches(method.id()))
      .and_then(|(index, method)| match method.as_ref() {
        MethodRef::Refer(did) => self.resolve(did.fragment()?),
        MethodRef::Embed(method) => Some(MethodWrap::new(method, index, query.scope)),
      })
  }

  fn resolve_verification_method(&self, query: MethodQuery) -> Option<MethodWrap<U>> {
    self
      .verification_method
      .iter()
      .enumerate()
      .find(|(index, method)| query.ident == *index || query.ident.matches(method.id()))
      .map(|(index, method)| MethodWrap::new(method, index, MethodScope::VerificationMethod))
  }
}

impl<T, U, V> Display for Document<T, U, V>
where
  T: Serialize,
  U: Serialize,
  V: Serialize,
{
  fn fmt(&self, f: &mut Formatter) -> FmtResult {
    if f.alternate() {
      f.write_str(&to_string_pretty(self).map_err(|_| FmtError)?)
    } else {
      f.write_str(&to_string(self).map_err(|_| FmtError)?)
    }
  }
}
