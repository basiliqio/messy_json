#[derive(Clone, Debug, PartialEq, Eq, Copy, Default)]
pub struct MessyJsonSettings {
    pub all_optional: bool,
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
