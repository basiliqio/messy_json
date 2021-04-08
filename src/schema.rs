use std::ops::Deref;

use super::*;

/// ## Schema of a JSON Value
///
/// This enum describes in broad strokes how a JSON should look like when deserialized.
///
/// At deserialization, this enum will ensure that the JSON Value corresponds to this schema.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum MessyJsonInner {
    Array(MessyJsonArray),
    Bool(MessyJsonScalar),
    Number(MessyJsonNumeric),
    Obj(MessyJsonObject),
    String(MessyJsonScalar),
    #[cfg(feature = "uuid")]
    Uuid(MessyJsonScalar),
}
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct MessyJson(Arc<MessyJsonInner>);

impl std::ops::Deref for MessyJson {
    type Target = MessyJsonInner;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl MessyJson {
    #[inline]
    pub fn builder(&self, all_optional: bool) -> MessyJsonBuilder {
        MessyJsonBuilder::new(self, all_optional)
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum MessyJsonExpected {
    Root(MessyJson),
    Obj(MessyJsonObject),
}

impl MessyJsonInner {
    /// Check if the inner value of this enum is optional
    pub fn optional(&self) -> bool {
        match self {
            MessyJsonInner::Array(x) => x.optional(),
            MessyJsonInner::Bool(x) => x.optional(),
            MessyJsonInner::Number(x) => x.optional(),
            MessyJsonInner::Obj(x) => x.optional(),
            MessyJsonInner::String(x) => x.optional(),
            #[cfg(feature = "uuid")]
            MessyJsonInner::Uuid(x) => x.optional(),
        }
    }
}

impl From<MessyJsonInner> for MessyJson {
    fn from(x: MessyJsonInner) -> Self {
        MessyJson(Arc::new(x))
    }
}

impl From<MessyJsonArray> for MessyJsonInner {
    fn from(x: MessyJsonArray) -> Self {
        MessyJsonInner::Array(x)
    }
}

impl From<MessyJsonNumeric> for MessyJsonInner {
    fn from(x: MessyJsonNumeric) -> Self {
        MessyJsonInner::Number(x)
    }
}

impl From<MessyJsonObject> for MessyJsonInner {
    fn from(x: MessyJsonObject) -> Self {
        MessyJsonInner::Obj(x)
    }
}

impl From<&MessyJsonObject> for MessyJsonInner {
    fn from(x: &MessyJsonObject) -> Self {
        MessyJsonInner::Obj(x.clone())
    }
}

impl From<&MessyJsonNumeric> for MessyJsonInner {
    fn from(x: &MessyJsonNumeric) -> Self {
        MessyJsonInner::Number(*x)
    }
}

/// ## Schema deserializer of a JSON Value
///
/// This struct takes a reference to a [MessyJson](MessyJson) and expose `serde`'s
/// deserialization trait.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MessyJsonBuilder {
    schema: MessyJson,
    all_optional: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MessyJsonObjectBuilder {
    schema: MessyJsonObject,
    all_optional: bool,
}

pub trait MessyJsonObjectTrait {
    type Input;

    /// Create a new builder from a [MessyJson](MessyJson)
    fn new(schema: &Self::Input, all_optional: bool) -> Self;

    /// Get the inner [MessyJson](MessyJson)
    fn inner(&self) -> &Self::Input;

    /// True if fields presence shouldn't be checked
    fn all_optional(&self) -> bool;

    /// Create a new nested schema providing the nested schema and self
    fn new_nested(&self, schema: &MessyJson, all_optional: bool) -> MessyJsonBuilder;

    /// Compare that a deserialized object have all the required fields are available.
    ///
    /// Return a missing key if any, None otherwise
    fn compare_obj(
        schema: &MessyJsonObject,
        res: &mut BTreeMap<ArcStr, MessyJsonValue>,
    ) -> Option<String> {
        let mut to_be_merged: BTreeMap<ArcStr, MessyJsonValue> = BTreeMap::new();
        let el = itertools::merge_join_by(schema.properties(), res.keys(), |(key1, _), key2| {
            Ord::cmp(key1.as_str(), key2)
        })
        .find(|merged| match merged {
            itertools::EitherOrBoth::Both(_, _) => false,
            itertools::EitherOrBoth::Left((key, val)) => match val.optional() {
                true => {
                    to_be_merged.insert(
                        (*key).clone(),
                        MessyJsonValue::Null(
                            MessyJsonNullType::Absent,
                            MessyJsonExpected::Root((*val).clone()),
                        ),
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

impl<'a> MessyJsonObjectTrait for MessyJsonBuilder {
    type Input = MessyJson;

    #[inline]
    fn new(schema: &Self::Input, all_optional: bool) -> Self {
        MessyJsonBuilder {
            schema: schema.clone(),
            all_optional,
        }
    }

    #[inline]
    fn inner(&self) -> &Self::Input {
        &self.schema
    }

    #[inline]
    fn all_optional(&self) -> bool {
        self.all_optional
    }

    #[inline]
    fn new_nested(&self, schema: &MessyJson, all_optional: bool) -> MessyJsonBuilder {
        MessyJsonBuilder {
            schema: schema.clone(),
            all_optional,
        }
    }
}

impl<'a> MessyJsonObjectTrait for MessyJsonObjectBuilder {
    type Input = MessyJsonObject;

    #[inline]
    fn new(schema: &Self::Input, all_optional: bool) -> Self {
        MessyJsonObjectBuilder {
            schema: schema.clone(),
            all_optional,
        }
    }

    #[inline]
    fn inner(&self) -> &Self::Input {
        &self.schema
    }

    #[inline]
    fn all_optional(&self) -> bool {
        self.all_optional
    }

    #[inline]
    fn new_nested(&self, schema: &MessyJson, all_optional: bool) -> MessyJsonBuilder {
        MessyJsonBuilder::new(schema, all_optional)
    }
}

impl<'de> DeserializeSeed<'de> for MessyJsonBuilder {
    type Value = MessyJsonValueContainer<'de>;
    #[inline]
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: Deserializer<'de>,
    {
        match self.inner().deref() {
            MessyJsonInner::Bool(opt) => match opt.optional() || self.all_optional() {
                true => deserializer.deserialize_option(self),
                false => deserializer.deserialize_bool(self),
            },
            MessyJsonInner::String(opt) => match opt.optional() || self.all_optional() {
                true => deserializer.deserialize_option(self),
                false => deserializer.deserialize_str(self),
            },
            MessyJsonInner::Number(opt) => match opt.optional() || self.all_optional() {
                true => deserializer.deserialize_option(self),
                false => match opt.type_() {
                    MessyJsonNumberType::U64 => deserializer.deserialize_u64(self),
                    MessyJsonNumberType::U128 => deserializer.deserialize_u128(self),
                },
            },
            MessyJsonInner::Obj(opt) => match opt.optional() || self.all_optional() {
                true => deserializer.deserialize_option(self),
                false => deserializer.deserialize_map(self),
            },
            MessyJsonInner::Array(opt) => match opt.optional() || self.all_optional() {
                true => deserializer.deserialize_option(self),
                false => deserializer.deserialize_seq(self),
            },
            #[cfg(feature = "uuid")]
            MessyJsonInner::Uuid(opt) => match opt.optional() || self.all_optional() {
                true => deserializer.deserialize_option(self),
                false => deserializer.deserialize_str(self),
            },
        }
    }
}

impl<'de> DeserializeSeed<'de> for MessyJsonObjectBuilder {
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
