use super::*;
use crate::schema::MessyJsonObjectTrait;

pub type KeyType = ArcStr;

#[derive(Clone, Debug, PartialEq, Eq, Hash, Default)]
pub struct MessyJsonObject(Arc<MessyJsonObjectInner>);

impl MessyJsonObject {
    #[inline]
    pub fn builder(&self, all_optional: bool) -> MessyJsonObjectBuilder {
        MessyJsonObjectBuilder::new(self, all_optional)
    }
}

impl std::ops::Deref for MessyJsonObject {
    type Target = MessyJsonObjectInner;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<MessyJsonObjectInner> for MessyJsonObject {
    fn from(x: MessyJsonObjectInner) -> Self {
        MessyJsonObject(Arc::new(x))
    }
}

/// ## JSON Object schema value
///
/// Describe a JSON Object at runtime specify if the object may be null and its
/// properties
#[derive(Clone, Debug, PartialEq, Eq, Hash, Default)]
pub struct MessyJsonObjectInner {
    optional: bool,
    properties: BTreeMap<KeyType, MessyJson>,
}

impl MessyJsonObjectInner {
    /// Create a new [MessyJsonObject](MessyJsonObject)
    pub fn new(properties: BTreeMap<KeyType, MessyJson>, optional: bool) -> Self {
        MessyJsonObjectInner {
            properties: properties.into_iter().collect(),
            optional,
        }
    }

    /// Get a reference to the [MessyJsonObject](MessyJsonObject)'s properties
    #[inline]
    pub fn properties(&self) -> &BTreeMap<KeyType, MessyJson> {
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

pub fn gen_key(k: &str) -> super::object::KeyType {
    ArcStr::from(k)
}
