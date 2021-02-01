use super::*;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MessyJsonArray {
    optional: bool,
    items: MessyJson,
}

impl MessyJsonArray {
    pub fn new(items: MessyJson, optional: bool) -> Self {
        MessyJsonArray { items, optional }
    }

    #[inline]
    pub fn items(&self) -> &MessyJson {
        &self.items
    }

    #[inline]
    pub fn optional(&self) -> bool {
        self.optional
    }
}
