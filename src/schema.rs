use super::*;

/// ## Schema of a JSON Value
///
/// This enum describes in broad strokes how a JSON should look like when deserialized.
///
/// At deserialization, this enum will ensure that the JSON Value corresponds to this schema.
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
    /// Return a builder, to deserialize an object with
    pub fn builder(&self) -> MessyJsonBuilder {
        MessyJsonBuilder::new(self)
    }

    /// Check if the inner value of this enum is optional
    pub fn optional(&self) -> bool {
        match self {
            MessyJson::Array(x) => x.optional(),
            MessyJson::Bool(x) => x.optional(),
            MessyJson::Number(x) => x.optional(),
            MessyJson::Obj(x) => x.optional(),
            MessyJson::String(x) => x.optional(),
            MessyJson::Null => true,
        }
    }
}

/// ## Schema deserializer of a JSON Value
///
/// This struct takes a reference to a [MessyJson](MessyJson) and expose `serde`'s
/// deserialization trait.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MessyJsonBuilder<'a> {
    schema: &'a MessyJson,
}

impl<'a> MessyJsonBuilder<'a> {
    /// Create a new builder from a [MessyJson](MessyJson)
    #[inline]
    fn new(schema: &'a MessyJson) -> Self {
        MessyJsonBuilder { schema }
    }

    /// Get the inner [MessyJson](MessyJson)
    #[inline]
    pub fn inner(&self) -> &'a MessyJson {
        &self.schema
    }

    /// Create a new nested schema providing the nested schema and self
    #[inline]
    pub(crate) fn new_nested(&self, schema: &'a MessyJson) -> Self {
        MessyJsonBuilder { schema }
    }

    /// Compare that a deserialized object have all the required fields are available.
    ///
    /// Return a missing key if any, None otherwise
    pub(crate) fn compare_obj(
        schema: &MessyJsonObject,
        res: &BTreeMap<Cow<'_, str>, MessyJsonValue>,
    ) -> Option<String> {
        let el = itertools::merge_join_by(schema.properties(), res.keys(), |(key1, _), key2| {
            Ord::cmp(key1.as_str(), key2)
        })
        .find(|merged| match merged {
            itertools::EitherOrBoth::Both(_, _) => false,
            itertools::EitherOrBoth::Left((_key, val)) => !val.optional(),
            itertools::EitherOrBoth::Right(_) => true,
        });
        el.map(|x| {
            match x {
                itertools::EitherOrBoth::Both(_, x) => x,
                itertools::EitherOrBoth::Left((key, _val)) => key.as_str(),
                itertools::EitherOrBoth::Right(x) => x,
            }
            .to_string()
        })
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
