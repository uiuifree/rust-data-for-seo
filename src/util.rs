use serde_json::Value;

pub(crate) struct LineUtil {}

impl LineUtil {
    pub(crate) fn value_to_value(key: &str, value: &Value) -> Option<Value> {
        Some(value.get(key)?.clone())
    }
    pub(crate) fn value_to_string(key: &str, value: &Value) -> Option<String> {
        match value.get(key)? {
            Value::String(v) => Some(v.to_string()),
            _ => None,
        }
    }
    // pub(crate) fn value_to_u32(key: &str, value: &Value) -> Option<u64> {
    //     match value.get(key)? {
    //         Value::Number(v) => Some(v.as_u64().unwrap_or_default()),
    //         _ => None,
    //     }
    // }
    pub(crate) fn value_to_array(key: &str, value: &Value) -> Option<Vec<Value>> {
        match value.get(key)? {
            Value::Array(v) => Some(v.to_vec()),
            _ => None,
        }
    }
    // pub(crate) fn value_to_boolean(key: &str, value: &Value) -> Option<bool> {
    //     match value.get(key)? {
    //         Value::Bool(v) => Some(v.clone()),
    //         _ => None,
    //     }
    // }
}
