use super::*;

#[derive(Clone, Debug, PartialEq, Eq, Getters)]
#[getset(get = "pub")]
pub struct MessyJsonScalar {
    pub optional: bool,
}

impl MessyJsonScalar {
    pub fn new(optional: bool) -> Self {
        MessyJsonScalar { optional }
    }
}
