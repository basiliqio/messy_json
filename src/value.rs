use super::*;
use serde_json::Value;
use std::convert::From;
use std::ops::Deref;

/// ## Deserialized JSON Object Value
#[derive(Clone, Debug, Eq, PartialEq, Default)]
pub struct MessyJsonObjectValue<'a>(BTreeMap<ArcStr, MessyJsonValue<'a>>);

/// ## Deserialized JSON Null Value
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum MessyJsonNullType {
    /// A field explicitly set to `null`
    Null,
    /// an absent field
    Absent,
}

impl<'a> Deref for MessyJsonObjectValue<'a> {
    type Target = BTreeMap<ArcStr, MessyJsonValue<'a>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'a> MessyJsonObjectValue<'a> {
    /// Take the inner value, consuming the object
    pub fn take(self) -> BTreeMap<ArcStr, MessyJsonValue<'a>> {
        self.0
    }
}

impl<'a> From<BTreeMap<ArcStr, MessyJsonValue<'a>>> for MessyJsonObjectValue<'a> {
    fn from(obj: BTreeMap<ArcStr, MessyJsonValue<'a>>) -> Self {
        MessyJsonObjectValue(obj)
    }
}

impl<'a> From<Vec<MessyJsonValue<'a>>> for MessyJsonArrayValue<'a> {
    fn from(arr: Vec<MessyJsonValue<'a>>) -> Self {
        MessyJsonArrayValue(arr)
    }
}

impl<'a> MessyJsonArrayValue<'a> {
    pub fn take(self) -> Vec<MessyJsonValue<'a>> {
        self.0
    }
}

/// ## Deserialized JSON Array Value
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MessyJsonArrayValue<'a>(Vec<MessyJsonValue<'a>>);

impl<'a> Deref for MessyJsonArrayValue<'a> {
    type Target = Vec<MessyJsonValue<'a>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// ## Deserialized JSON Value
///
/// This structure holds references to the deserialized data.
///
/// It is structured like [Serde JSON's `Value`](serde_json::Value).
///
/// Every string is borrowed rather than cloned
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MessyJsonValue<'a> {
    Array(MessyJsonArrayValue<'a>),
    Bool(bool),
    Number(u128),
    Obj(MessyJsonObjectValue<'a>),
    String(Cow<'a, str>),
    #[cfg(feature = "uuid")]
    Uuid(Cow<'a, feat_uuid::Uuid>),
    Null(MessyJsonNullType, MessyJsonExpected),
}

impl<'a> PartialEq<Value> for MessyJsonObjectValue<'a> {
    fn eq(&self, other: &Value) -> bool {
        match other {
            Value::Object(v_obj) => {
                for (k, v) in self.iter() {
                    if let Some(x) = v_obj.get(k.as_str()) {
                        if v != x {
                            return false;
                        }
                        continue;
                    }
                    return false;
                }
                true
            }
            _ => false,
        }
    }
}

impl<'a> PartialEq<Value> for MessyJsonArrayValue<'a> {
    fn eq(&self, other: &Value) -> bool {
        match other {
            Value::Array(v_arr) => self.0.eq(v_arr),
            _ => false,
        }
    }
}

#[cfg(feature = "uuid")]
impl<'a> PartialEq<feat_uuid::Uuid> for MessyJsonValue<'a> {
    fn eq(&self, other: &feat_uuid::Uuid) -> bool {
        match self {
            MessyJsonValue::Uuid(v_uuid) => *v_uuid == Cow::Borrowed(other),
            _ => false,
        }
    }
}

impl<'a> PartialEq<Value> for MessyJsonValue<'a> {
    fn eq(&self, other: &Value) -> bool {
        match (self, other) {
            (MessyJsonValue::Array(mj_arr), Value::Array(_)) => mj_arr.eq(other),
            (MessyJsonValue::Bool(mj_bool), Value::Bool(v_bool)) => mj_bool == v_bool,
            (MessyJsonValue::Number(mj_number), Value::Number(v_number)) => {
                let num = match v_number.as_u64() {
                    Some(x) => x,
                    None => return false, // TODO Handle better
                };
                mj_number == &(num as u128)
            }
            (MessyJsonValue::Obj(mj_obj), Value::Object(_)) => mj_obj.eq(other),
            (MessyJsonValue::String(mj_str), Value::String(v_str)) => mj_str == v_str,
            (MessyJsonValue::Null(_, _), Value::Null) => true,
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
