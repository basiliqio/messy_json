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
