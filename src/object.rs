use super::*;

#[derive(Clone, Debug, PartialEq, Eq, Getters)]
#[getset(get = "pub")]
pub struct MessyJsonObject {
    optional: bool,
    properties: HashMap<String, MessyJson>,
}

impl MessyJsonObject {
    pub fn new(properties: HashMap<String, MessyJson>, optional: bool) -> Self {
        MessyJsonObject {
            properties,
            optional,
        }
    }
}
