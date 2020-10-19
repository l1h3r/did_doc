use alloc::collections::BTreeMap;
use alloc::string::String;

pub type Object = BTreeMap<String, Value>;
pub type Value = serde_json::Value;
