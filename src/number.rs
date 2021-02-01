use super::*;

#[derive(Clone, Debug, PartialEq, Eq, Getters)]
#[getset(get = "pub")]
pub struct MessyJsonNumeric {
    optional: bool,
    type_: MessyJsonNumberType,
}

impl MessyJsonNumeric {
    pub fn new(type_: MessyJsonNumberType, optional: bool) -> Self {
        MessyJsonNumeric { type_, optional }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MessyJsonNumberType {
    U64,
    U128,
}

impl Default for MessyJsonNumberType {
    fn default() -> Self {
        MessyJsonNumberType::U64
    }
}
