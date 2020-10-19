use alloc::vec::Vec;
use did::DID;
use url::Url;

use crate::service::Service;
use crate::utils::Object;
use crate::utils::OrderedSet;
use crate::verification::Method;
use crate::verification::MethodIndex;
use crate::verification::MethodRef;
use crate::verification::MethodScope;
use crate::verification::MethodWrap;

#[derive(Clone, Debug, PartialEq)]
#[derive(Deserialize, Serialize)]
#[rustfmt::skip]
pub struct Document {
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
  pub(crate) properties: Object,
}

impl Document {
  pub fn id(&self) -> &DID {
    &self.id
  }

  pub fn controller(&self) -> Option<&DID> {
    self.controller.as_ref()
  }

  pub fn also_known_as(&self) -> &[Url] {
    &self.also_known_as
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

  pub fn properties(&self) -> &Object {
    &self.properties
  }

  pub fn properties_mut(&mut self) -> &mut Object {
    &mut self.properties
  }

  pub fn resolve_method<'a, T, U>(&self, ident: T, scope: U) -> Option<MethodWrap>
  where
    T: Into<MethodIndex<'a>>,
    U: Into<Option<MethodScope>>,
  {
    self.resolve_method_(ident.into(), scope.into().unwrap_or_default())
  }

  fn resolve_method_(&self, ident: MethodIndex, scope: MethodScope) -> Option<MethodWrap> {
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
      .find(|(index, method)| ident == *index || Self::matches_fragment(method.id(), ident))
      .and_then(|(index, method)| match method {
        MethodRef::Refer(did) => self.resolve_method(did.fragment().expect("infallible"), None),
        MethodRef::Embed(method) => Some(MethodWrap::new(index, method)),
      })
  }

  fn resolve_verification_method(&self, ident: MethodIndex) -> Option<MethodWrap> {
    self
      .verification_method
      .iter()
      .enumerate()
      .find(|(index, method)| ident == *index || Self::matches_fragment(method.id(), ident))
      .map(|(index, method)| MethodWrap::new(index, method))
  }

  fn matches_fragment(did: &DID, ident: MethodIndex) -> bool {
    matches!(did.fragment(), Some(fragment) if ident == fragment)
  }
}
