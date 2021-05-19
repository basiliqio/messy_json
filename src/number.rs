/// ## JSON Number schema value
///
/// Describe a JSON Number at runtime. The type of number is to differentiate normal
/// `u64` value from bigger `u128` (and more expensive) numbers.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct MessyJsonNumeric {
    optional: bool,
    type_: MessyJsonNumberType,
}

impl MessyJsonNumeric {
    /// Create a new [MessyJsonNumeric](MessyJsonNumeric)
    pub fn new(type_: MessyJsonNumberType, optional: bool) -> Self {
        MessyJsonNumeric { optional, type_ }
    }
}

/// ## JSON Number type schema
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum MessyJsonNumberType {
    U64,
    U128,
}

impl Default for MessyJsonNumberType {
    fn default() -> Self {
        MessyJsonNumberType::U64
    }
}

impl MessyJsonNumeric {
    /// ## Get the type of number
    #[inline]
    pub fn type_(&self) -> MessyJsonNumberType {
        self.type_
    }

    /// ## Check if the number is optional
    #[inline]
    pub fn optional(&self) -> bool {
        self.optional
    }
}
