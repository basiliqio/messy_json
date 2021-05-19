use super::*;

mod deserialize;
mod deserializer;

pub use deserialize::MessyJsonValueRawVisitor;

/// ## Deserialized JSON Value
///
/// This structure holds references to the deserialized data.
///
/// It is structured like [Serde JSON's `Value`](serde_json::Value).
///
/// Every string is borrowed rather than cloned
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MessyJsonValueRaw<'a> {
    Array(Vec<MessyJsonValueRaw<'a>>),
    Bool(bool),
    Number(u128),
    Obj(BTreeMap<Cow<'a, str>, MessyJsonValueRaw<'a>>),
    String(Cow<'a, str>),
    Null,
}

impl<'a> From<bool> for MessyJsonValueRaw<'a> {
    fn from(val: bool) -> Self {
        MessyJsonValueRaw::Bool(val)
    }
}

impl<'a> From<i64> for MessyJsonValueRaw<'a> {
    fn from(val: i64) -> Self {
        MessyJsonValueRaw::Number(val as u128)
    }
}

impl<'a> From<u64> for MessyJsonValueRaw<'a> {
    fn from(val: u64) -> Self {
        MessyJsonValueRaw::Number(val as u128)
    }
}

impl<'a> From<f64> for MessyJsonValueRaw<'a> {
    fn from(val: f64) -> Self {
        MessyJsonValueRaw::Number(val as u128)
    }
}

impl<'a> From<&'a str> for MessyJsonValueRaw<'a> {
    fn from(val: &'a str) -> Self {
        MessyJsonValueRaw::String(Cow::Borrowed(val))
    }
}

impl<'a> From<String> for MessyJsonValueRaw<'a> {
    fn from(val: String) -> Self {
        MessyJsonValueRaw::String(Cow::Owned(val))
    }
}

#[allow(clippy::from_over_into)]
impl<'a> Into<serde::de::Unexpected<'a>> for MessyJsonValueRaw<'a> {
    fn into(self) -> serde::de::Unexpected<'a> {
        match self {
            MessyJsonValueRaw::Bool(x) => serde::de::Unexpected::Bool(x),
            MessyJsonValueRaw::Number(x) => serde::de::Unexpected::Unsigned(x as u64),
            MessyJsonValueRaw::String(_) => serde::de::Unexpected::StructVariant,
            MessyJsonValueRaw::Array(_) => serde::de::Unexpected::Seq,
            MessyJsonValueRaw::Obj(_) => serde::de::Unexpected::Map,
            MessyJsonValueRaw::Null => serde::de::Unexpected::Unit,
        }
    }
}
