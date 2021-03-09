/// ## JSON Scalar schema value
///
/// Simple struct to specifiy if a JSON scalar value is optional or not
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct MessyJsonScalar {
    pub optional: bool,
}

impl MessyJsonScalar {
    /// Create a new [MessyJsonScalar](MessyJsonScalar)
    pub fn new(optional: bool) -> Self {
        MessyJsonScalar { optional }
    }

    /// Check if the scalar value is optional
    #[inline]
    pub fn optional(&self) -> bool {
        self.optional
    }
}
