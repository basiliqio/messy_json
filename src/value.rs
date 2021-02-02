use super::*;

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
