use alloc::vec::Vec;
use core::fmt::Display;
use core::fmt::Error;
use core::fmt::Formatter;
use core::fmt::Result;
use did_url::DID;
use serde::Serialize;
use serde_json::to_string;
use serde_json::to_string_pretty;
use url::Url;

use crate::service::Service;
use crate::utils::Object;
use crate::utils::OrderedSet;
use crate::verification::Method;
use crate::verification::MethodIndex;
use crate::verification::MethodRef;
use crate::verification::MethodScope;
use crate::verification::MethodWrap;

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[rustfmt::skip]
pub struct Document<T = Object> {
  pub(crate) id: DID,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub(crate) controller: Option<DID>,
  #[serde(default, rename = "alsoKnownAs", skip_serializing_if = "Vec::is_empty")]
  pub(crate) also_known_as: Vec<Url>,
  #[serde(default, rename = "verificationMethod", skip_serializing_if = "OrderedSet::is_empty")]
  pub(crate) verification_method: OrderedSet<Method>,
  #[serde(default, skip_serializing_if = "OrderedSet::is_empty")]
  pub(crate) authentication: OrderedSet<MethodRef>,
  #[serde(default, rename = "assertionMethod", skip_serializing_if = "OrderedSet::is_empty")]
  pub(crate) assertion_method: OrderedSet<MethodRef>,
  #[serde(default, rename = "keyAgreement", skip_serializing_if = "OrderedSet::is_empty")]
  pub(crate) key_agreement: OrderedSet<MethodRef>,
  #[serde(default, rename = "capabilityDelegation", skip_serializing_if = "OrderedSet::is_empty")]
  pub(crate) capability_delegation: OrderedSet<MethodRef>,
  #[serde(default, rename = "capabilityInvocation", skip_serializing_if = "OrderedSet::is_empty")]
  pub(crate) capability_invocation: OrderedSet<MethodRef>,
  #[serde(default, skip_serializing_if = "Vec::is_empty")]
  pub(crate) service: Vec<Service>,
  #[serde(flatten)]
  pub(crate) properties: T,
}

impl<T> Document<T> {
  pub fn id(&self) -> &DID {
    &self.id
  }

  pub fn id_mut(&mut self) -> &mut DID {
    &mut self.id
  }

  pub fn controller(&self) -> Option<&DID> {
    self.controller.as_ref()
  }

  pub fn controller_mut(&mut self) -> Option<&mut DID> {
    self.controller.as_mut()
  }

  pub fn also_known_as(&self) -> &[Url] {
    &self.also_known_as
  }

  pub fn also_known_as_mut(&mut self) -> &mut Vec<Url> {
    &mut self.also_known_as
  }

  pub fn verification_method(&self) -> &OrderedSet<Method> {
    &self.verification_method
  }

  pub fn verification_method_mut(&mut self) -> &mut OrderedSet<Method> {
    &mut self.verification_method
  }

  pub fn authentication(&self) -> &OrderedSet<MethodRef> {
    &self.authentication
  }

  pub fn authentication_mut(&mut self) -> &mut OrderedSet<MethodRef> {
    &mut self.authentication
  }

  pub fn assertion_method(&self) -> &OrderedSet<MethodRef> {
    &self.assertion_method
  }

  pub fn assertion_method_mut(&mut self) -> &mut OrderedSet<MethodRef> {
    &mut self.assertion_method
  }

  pub fn key_agreement(&self) -> &OrderedSet<MethodRef> {
    &self.key_agreement
  }

  pub fn key_agreement_mut(&mut self) -> &mut OrderedSet<MethodRef> {
    &mut self.key_agreement
  }

  pub fn capability_delegation(&self) -> &OrderedSet<MethodRef> {
    &self.capability_delegation
  }

  pub fn capability_delegation_mut(&mut self) -> &mut OrderedSet<MethodRef> {
    &mut self.capability_delegation
  }

  pub fn capability_invocation(&self) -> &OrderedSet<MethodRef> {
    &self.capability_invocation
  }

  pub fn capability_invocation_mut(&mut self) -> &mut OrderedSet<MethodRef> {
    &mut self.capability_invocation
  }

  pub fn service(&self) -> &[Service] {
    &self.service
  }

  pub fn service_mut(&mut self) -> &mut Vec<Service> {
    &mut self.service
  }

  pub fn properties(&self) -> &T {
    &self.properties
  }

  pub fn properties_mut(&mut self) -> &mut T {
    &mut self.properties
  }

  pub fn map<U, F>(self, f: F) -> Document<U>
  where
    F: FnOnce(T) -> U,
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

  pub fn resolve<'a, I, S>(&self, ident: I, scope: S) -> Option<MethodWrap>
  where
    I: Into<MethodIndex<'a>>,
    S: Into<Option<MethodScope>>,
  {
    self.resolve_method(ident.into(), scope.into().unwrap_or_default())
  }

  fn resolve_method(&self, ident: MethodIndex, scope: MethodScope) -> Option<MethodWrap> {
    let iter = match scope {
      MethodScope::VerificationMethod => return self.resolve_verification_method(ident),
      MethodScope::Authentication => self.authentication.iter(),
      MethodScope::AssertionMethod => self.assertion_method.iter(),
      MethodScope::KeyAgreement => self.key_agreement.iter(),
      MethodScope::CapabilityDelegation => self.capability_delegation.iter(),
      MethodScope::CapabilityInvocation => self.capability_invocation.iter(),
    };

    iter
      .enumerate()
      .find(|(index, method)| ident == *index || ident.matches(method.id()))
      .and_then(|(index, method)| match method {
        MethodRef::Refer(did) => self.resolve(did.fragment()?, None),
        MethodRef::Embed(method) => Some(MethodWrap::new(index, method)),
      })
  }

  fn resolve_verification_method(&self, ident: MethodIndex) -> Option<MethodWrap> {
    self
      .verification_method
      .iter()
      .enumerate()
      .find(|(index, method)| ident == *index || ident.matches(method.id()))
      .map(|(index, method)| MethodWrap::new(index, method))
  }
}

impl<T> Display for Document<T>
where
  T: Serialize,
{
  fn fmt(&self, f: &mut Formatter) -> Result {
    if f.alternate() {
      f.write_str(&to_string_pretty(self).map_err(|_| Error)?)
    } else {
      f.write_str(&to_string(self).map_err(|_| Error)?)
    }
  }
}
