use super::*;
use crate::schema::MessyJsonObjectTrait;

/// ## JSON Object schema value
///
/// Describe a JSON Object at runtime specify if the object may be null and its
/// properties
#[derive(Clone, Debug, PartialEq, Eq, Hash, Default)]
pub struct MessyJsonObject<'a> {
    optional: bool,
    properties: BTreeMap<String, MessyJson<'a>>,
}

impl<'a> MessyJsonObject<'a> {
    /// Create a new [MessyJsonObject](MessyJsonObject)
    pub fn new(properties: BTreeMap<String, MessyJson<'a>>, optional: bool) -> Self {
        MessyJsonObject {
            properties,
            optional,
        }
    }

    #[inline]
    pub fn builder(&'a self, all_optional: bool) -> MessyJsonObjectBuilder<'a> {
        MessyJsonObjectBuilder::new(self, all_optional)
    }

    /// Get a reference to the [MessyJsonObject](MessyJsonObject)'s properties
    #[inline]
    pub fn properties(&self) -> &BTreeMap<String, MessyJson<'a>> {
        &self.properties
    }

    /// Get a reference to the [MessyJsonObject](MessyJsonObject)'s properties
    #[inline]
    pub fn has_field(&self, key: &str) -> bool {
        self.properties.contains_key(key)
    }

    /// Check if the object is optional
    #[inline]
    pub fn optional(&self) -> bool {
        self.optional
    }
}
