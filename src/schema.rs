use super::*;

/// ## Schema of a JSON Value
///
/// This enum describes in broad strokes how a JSON should look like when deserialized.
///
/// At deserialization, this enum will ensure that the JSON Value corresponds to this schema.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum MessyJson<'a> {
    Array(Box<MessyJsonArray<'a>>),
    Bool(Cow<'a, MessyJsonScalar>),
    Number(Cow<'a, MessyJsonNumeric>),
    Obj(Cow<'a, MessyJsonObject<'a>>),
    String(Cow<'a, MessyJsonScalar>),
    #[cfg(feature = "uuid")]
    Uuid(Cow<'a, MessyJsonScalar>),
}

impl<'a> MessyJson<'a> {
    /// Return a builder, to deserialize an object with
    pub fn builder(&'a self, all_optional: bool) -> MessyJsonBuilder<'a> {
        MessyJsonBuilder::new(&self, all_optional)
    }

    /// Check if the inner value of this enum is optional
    pub fn optional(&'a self) -> bool {
        match self {
            MessyJson::Array(x) => x.optional(),
            MessyJson::Bool(x) => x.optional(),
            MessyJson::Number(x) => x.optional(),
            MessyJson::Obj(x) => x.optional(),
            MessyJson::String(x) => x.optional(),
            #[cfg(feature = "uuid")]
            MessyJson::Uuid(x) => x.optional(),
        }
    }
}

impl<'a> From<MessyJsonArray<'a>> for MessyJson<'a> {
    fn from(x: MessyJsonArray<'a>) -> Self {
        MessyJson::Array(Box::new(x))
    }
}

impl<'a> From<MessyJsonNumeric> for MessyJson<'a> {
    fn from(x: MessyJsonNumeric) -> Self {
        MessyJson::Number(Cow::Owned(x))
    }
}

impl<'a> From<MessyJsonObject<'a>> for MessyJson<'a> {
    fn from(x: MessyJsonObject<'a>) -> Self {
        MessyJson::Obj(Cow::Owned(x))
    }
}

impl<'a> From<&'a MessyJsonObject<'a>> for MessyJson<'a> {
    fn from(x: &'a MessyJsonObject<'a>) -> Self {
        MessyJson::Obj(Cow::Borrowed(x))
    }
}

impl<'a> From<&'a MessyJsonNumeric> for MessyJson<'a> {
    fn from(x: &'a MessyJsonNumeric) -> Self {
        MessyJson::Number(Cow::Borrowed(x))
    }
}

/// ## Schema deserializer of a JSON Value
///
/// This struct takes a reference to a [MessyJson](MessyJson) and expose `serde`'s
/// deserialization trait.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MessyJsonBuilder<'a> {
    schema: &'a MessyJson<'a>,
    all_optional: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MessyJsonObjectBuilder<'a> {
    schema: &'a MessyJsonObject<'a>,
    all_optional: bool,
}

pub trait MessyJsonObjectTrait<'a> {
    type Input;

    /// Create a new builder from a [MessyJson](MessyJson)
    fn new(schema: &'a Self::Input, all_optional: bool) -> Self;

    /// Get the inner [MessyJson](MessyJson)
    fn inner(&self) -> &'a Self::Input;

    /// True if fields presence shouldn't be checked
    fn all_optional(&self) -> bool;

    /// Create a new nested schema providing the nested schema and self
    fn new_nested(&self, schema: &'a MessyJson<'a>, all_optional: bool) -> MessyJsonBuilder<'a>;

    /// Compare that a deserialized object have all the required fields are available.
    ///
    /// Return a missing key if any, None otherwise
    fn compare_obj(
        schema: &'a MessyJsonObject<'a>,
        res: &mut BTreeMap<Cow<'a, str>, MessyJsonValue<'a>>,
    ) -> Option<String> {
        let mut to_be_merged: BTreeMap<Cow<'a, str>, MessyJsonValue<'a>> = BTreeMap::new();
        let el = itertools::merge_join_by(schema.properties(), res.keys(), |(key1, _), key2| {
            Ord::cmp(key1.as_str(), key2)
        })
        .find(|merged| match merged {
            itertools::EitherOrBoth::Both(_, _) => false,
            itertools::EitherOrBoth::Left((key, val)) => match val.optional() {
                true => {
                    to_be_merged.insert(
                        Cow::Borrowed(key),
                        MessyJsonValue::Null(MessyJsonNullType::Absent, Cow::Borrowed(val)),
                    );
                    false
                }
                false => true,
            },
            itertools::EitherOrBoth::Right(_) => true,
        });
        let missing_key = el.map(|x| {
            match x {
                itertools::EitherOrBoth::Both(_, x) => x,
                itertools::EitherOrBoth::Left((key, _val)) => key.as_str(),
                itertools::EitherOrBoth::Right(x) => x,
            }
            .to_string()
        });
        res.append(&mut to_be_merged);
        missing_key
    }
}

impl<'a> MessyJsonObjectTrait<'a> for MessyJsonBuilder<'a> {
    type Input = MessyJson<'a>;

    #[inline]
    fn new(schema: &'a Self::Input, all_optional: bool) -> Self {
        MessyJsonBuilder {
            schema,
            all_optional,
        }
    }

    #[inline]
    fn inner(&self) -> &'a Self::Input {
        self.schema
    }

    #[inline]
    fn all_optional(&self) -> bool {
        self.all_optional
    }

    #[inline]
    fn new_nested(&self, schema: &'a MessyJson<'a>, all_optional: bool) -> MessyJsonBuilder<'a> {
        MessyJsonBuilder {
            schema,
            all_optional,
        }
    }
}

impl<'a> MessyJsonObjectTrait<'a> for MessyJsonObjectBuilder<'a> {
    type Input = MessyJsonObject<'a>;

    #[inline]
    fn new(schema: &'a Self::Input, all_optional: bool) -> Self {
        MessyJsonObjectBuilder {
            schema,
            all_optional,
        }
    }

    #[inline]
    fn inner(&self) -> &'a Self::Input {
        self.schema
    }

    #[inline]
    fn all_optional(&self) -> bool {
        self.all_optional
    }

    #[inline]
    fn new_nested(&self, schema: &'a MessyJson<'a>, all_optional: bool) -> MessyJsonBuilder<'a> {
        MessyJsonBuilder::new(schema, all_optional)
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
            MessyJson::Bool(opt) => match opt.optional() || self.all_optional() {
                true => deserializer.deserialize_option(self),
                false => deserializer.deserialize_bool(self),
            },
            MessyJson::String(opt) => match opt.optional() || self.all_optional() {
                true => deserializer.deserialize_option(self),
                false => deserializer.deserialize_str(self),
            },
            MessyJson::Number(opt) => match opt.optional() || self.all_optional() {
                true => deserializer.deserialize_option(self),
                false => match opt.type_() {
                    MessyJsonNumberType::U64 => deserializer.deserialize_u64(self),
                    MessyJsonNumberType::U128 => deserializer.deserialize_u128(self),
                },
            },
            MessyJson::Obj(opt) => match opt.optional() || self.all_optional() {
                true => deserializer.deserialize_option(self),
                false => deserializer.deserialize_map(self),
            },
            MessyJson::Array(opt) => match opt.optional() || self.all_optional() {
                true => deserializer.deserialize_option(self),
                false => deserializer.deserialize_seq(self),
            },
            #[cfg(feature = "uuid")]
            MessyJson::Uuid(opt) => match opt.optional() || self.all_optional() {
                true => deserializer.deserialize_option(self),
                false => deserializer.deserialize_str(self),
            },
        }
    }
}

impl<'de> DeserializeSeed<'de> for MessyJsonObjectBuilder<'de> {
    type Value = MessyJsonValueContainer<'de>;
    #[inline]
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: Deserializer<'de>,
    {
        match self.inner().optional() {
            true => deserializer.deserialize_option(self),
            false => deserializer.deserialize_map(self),
        }
    }
}
