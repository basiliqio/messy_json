use super::*;

/// ## JSON Array schema value
///
/// Runtime representation of a 'to-be-deserialized' JSON Array.
///
/// This object specify if the array is optional and describes its children.
///
///
/// ### Limitations
///
/// This objects cannot describe multiple types of childrens
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct MessyJsonArray {
    optional: bool,
    items: MessyJson,
}

impl MessyJsonArray {
    /// Create a new [MessyJsonArray](MessyJsonArray)
    pub fn new(items: MessyJson, optional: bool) -> Self {
        MessyJsonArray { items, optional }
    }

    /// Get the underlying items of a [MessyJsonArray](MessyJsonArray)
    #[inline]
    pub fn items(&self) -> &MessyJson {
        &self.items
    }

    /// Check if the array is optional
    #[inline]
    pub fn optional(&self) -> bool {
        self.optional
    }
}
