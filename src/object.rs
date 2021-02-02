use super::*;

/// ## JSON Object schema value
///
/// Describe a JSON Object at runtime specify if the object may be null and its
/// properties
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MessyJsonObject {
    optional: bool,
    properties: BTreeMap<String, MessyJson>,
}

impl MessyJsonObject {
    /// Create a new [MessyJsonObject](MessyJsonObject)
    pub fn new(properties: BTreeMap<String, MessyJson>, optional: bool) -> Self {
        MessyJsonObject {
            properties,
            optional,
        }
    }

    /// Get a reference to the [MessyJsonObject](MessyJsonObject)'s properties
    #[inline]
    pub fn properties(&self) -> &BTreeMap<String, MessyJson> {
        &self.properties
    }

    /// Check if the object is optional
    #[inline]
    pub fn optional(&self) -> bool {
        self.optional
    }
}
