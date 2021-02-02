#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct MessyJsonScalar {
    pub optional: bool,
}

impl MessyJsonScalar {
    pub fn new(optional: bool) -> Self {
        MessyJsonScalar { optional }
    }

    #[inline]
    pub fn optional(&self) -> bool {
        self.optional
    }
}
