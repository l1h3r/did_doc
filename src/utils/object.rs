use crate::lib::*;

pub type Object = BTreeMap<String, Value>;
pub type Value = serde_json::Value;
