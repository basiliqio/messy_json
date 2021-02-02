use super::*;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MessyJsonValue<'a> {
    Array(Vec<MessyJsonValue<'a>>),
    Bool(bool),
    Number(u128),
    Obj(BTreeMap<Cow<'a, str>, MessyJsonValue<'a>>),
    String(Cow<'a, str>),
    Null,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MessyJsonValueContainer<'a> {
    val: MessyJsonValue<'a>,
}

impl<'a> std::ops::Deref for MessyJsonValueContainer<'a> {
    type Target = MessyJsonValue<'a>;

    #[inline]
    fn deref(&self) -> &Self::Target {
        self.val()
    }
}

impl<'a> MessyJsonValueContainer<'a> {
    #[inline]
    pub fn new(val: MessyJsonValue<'a>) -> Self {
        MessyJsonValueContainer { val }
    }

    #[inline]
    pub fn val(&self) -> &MessyJsonValue<'a> {
        &self.val
    }

    #[inline]
    pub fn take(self) -> MessyJsonValue<'a> {
        self.val
    }

    #[inline]
    pub fn val_mut(&mut self) -> &mut MessyJsonValue<'a> {
        &mut self.val
    }
}
