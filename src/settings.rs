/// Setting object for deserializing
#[derive(Clone, Debug, PartialEq, Eq, Copy, Default)]
pub struct MessyJsonSettings {
    /// True if all field should be considered optional.
    pub all_optional: bool,
    /// Ensure that mandatory field are not set to null explicitely when deserializing.
    ///
    /// Ignored if `all_optional` is `false`
    pub preserve_mandatory: bool,
}

impl MessyJsonSettings {
    pub fn all_optional(&self) -> bool {
        self.all_optional
    }

    pub fn preserve_mandatory(&self) -> bool {
        self.preserve_mandatory
    }
}
