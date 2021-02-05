use super::*;
use serde_json::Value;

/// ## Deserialized JSON Value
///
/// This structure holds references to the deserialized data.
///
/// It is structured like [Serde JSON's `Value`](serde_json::Value).
///
/// Every string is borrowed rather than cloned
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MessyJsonValue<'a> {
    Array(Vec<MessyJsonValue<'a>>),
    Bool(bool),
    Number(u128),
    Obj(BTreeMap<Cow<'a, str>, MessyJsonValue<'a>>),
    String(Cow<'a, str>),
    Null,
}

impl<'a> PartialEq<Value> for MessyJsonValue<'a> {
    fn eq(&self, other: &Value) -> bool {
        match (self, other) {
            (MessyJsonValue::Array(mj_arr), Value::Array(v_arr)) => mj_arr == v_arr,
            (MessyJsonValue::Bool(mj_bool), Value::Bool(v_bool)) => mj_bool == v_bool,
            (MessyJsonValue::Number(mj_number), Value::Number(v_number)) => {
                let num = match v_number.as_u64() {
                    Some(x) => x,
                    None => return false, // TODO Handle better
                };
                mj_number == &(num as u128)
            }
            (MessyJsonValue::Obj(mj_obj), Value::Object(v_obj)) => {
                for (k, v) in mj_obj.iter() {
                    if let Some(x) = v_obj.get(k.as_ref()) {
                        if v != x {
                            return false;
                        }
                        continue;
                    }
                    return false;
                }
                true
            }
            (MessyJsonValue::String(mj_str), Value::String(v_str)) => mj_str == v_str,
            (MessyJsonValue::Null, Value::Null) => true,
            _ => false,
        }
    }
}

/// ## Container for [MessyJsonValue](MessyJsonValue)
///
/// This structure is a simple wrapper around [MessyJsonValue](MessyJsonValue).
///
/// In the future it may be used to hold additional data for optimization purposes.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MessyJsonValueContainer<'a> {
    val: MessyJsonValue<'a>,
}

impl<'a> std::ops::Deref for MessyJsonValueContainer<'a> {
    type Target = MessyJsonValue<'a>;

    #[inline]
    fn deref(&self) -> &Self::Target {
        self.inner()
    }
}

impl<'a> MessyJsonValueContainer<'a> {
    /// Create a new [MessyJsonValueContainer](MessyJsonValueContainer) using [MessyJsonValue](MessyJsonValue)
    #[inline]
    pub fn new(val: MessyJsonValue<'a>) -> Self {
        MessyJsonValueContainer { val }
    }

    /// Get the inner [MessyJsonValue](MessyJsonValue)
    #[inline]
    pub fn inner(&self) -> &MessyJsonValue<'a> {
        &self.val
    }

    /// Take the inner [MessyJsonValue](MessyJsonValue) consuming the container
    #[inline]
    pub fn take(self) -> MessyJsonValue<'a> {
        self.val
    }
}
