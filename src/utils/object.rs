use alloc::collections::BTreeMap;
use alloc::string::String;

use crate::utils::Value;

pub type Object = BTreeMap<String, Value>;
