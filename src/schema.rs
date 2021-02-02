use super::*;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MessyJson {
    Array(Box<MessyJsonArray>),
    Bool(MessyJsonScalar),
    Number(MessyJsonNumeric),
    Obj(Box<MessyJsonObject>),
    String(MessyJsonScalar),
    Null,
}

impl MessyJson {
    pub fn builder<'a>(&'a self) -> MessyJsonBuilder<'a> {
        MessyJsonBuilder::new(self)
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MessyJsonBuilder<'a> {
    schema: &'a MessyJson,
}

impl<'a> MessyJsonBuilder<'a> {
    #[inline]
    fn new(schema: &'a MessyJson) -> Self {
        MessyJsonBuilder { schema }
    }

    #[inline]
    pub fn inner(&self) -> &'a MessyJson {
        &self.schema
    }

    #[inline]
    pub(crate) fn new_nested(&self, schema: &'a MessyJson) -> Self {
        MessyJsonBuilder { schema }
    }
}

impl<'de> DeserializeSeed<'de> for MessyJsonBuilder<'de> {
    type Value = MessyJsonValueContainer<'de>;
    #[inline]
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: Deserializer<'de>,
    {
        match self.inner() {
            MessyJson::Bool(opt) => match opt.optional() {
                true => deserializer.deserialize_option(self),
                false => deserializer.deserialize_bool(self),
            },
            MessyJson::String(opt) => match opt.optional() {
                true => deserializer.deserialize_option(self),
                false => deserializer.deserialize_str(self),
            },
            MessyJson::Number(opt) => match opt.optional() {
                true => deserializer.deserialize_option(self),
                false => match opt.type_() {
                    MessyJsonNumberType::U64 => deserializer.deserialize_u64(self),
                    MessyJsonNumberType::U128 => deserializer.deserialize_u128(self),
                },
            },
            MessyJson::Obj(opt) => match opt.optional() {
                true => deserializer.deserialize_option(self),
                false => deserializer.deserialize_map(self),
            },
            MessyJson::Array(opt) => match opt.optional() {
                true => deserializer.deserialize_option(self),
                false => deserializer.deserialize_seq(self),
            },
            MessyJson::Null => deserializer.deserialize_option(self),
        }
    }
}
