use super::*;

#[derive(Clone, Debug, PartialEq, Eq, Getters)]
#[getset(get = "pub")]
pub struct MessyJsonArray {
    optional: bool,
    items: MessyJson,
}

impl MessyJsonArray {
    pub fn new(items: MessyJson, optional: bool) -> Self {
        MessyJsonArray { items, optional }
    }
}
