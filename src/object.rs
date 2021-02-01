use super::*;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MessyJsonObject {
    optional: bool,
    properties: BTreeMap<String, MessyJson>,
}

impl MessyJsonObject {
    pub fn new(properties: BTreeMap<String, MessyJson>, optional: bool) -> Self {
        MessyJsonObject {
            properties,
            optional,
        }
    }

    #[inline]
    pub fn properties(&self) -> &BTreeMap<String, MessyJson> {
        &self.properties
    }

    #[inline]
    pub fn optional(&self) -> bool {
        self.optional
    }
}
