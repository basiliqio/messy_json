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
    pub fn builder(&self) -> MessyJsonBuilder {
        MessyJsonBuilder::new(self)
    }

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

    pub(crate) fn compare_obj(
        &self,
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
        //     if !is_done {
        //         if let Some(val_key) = res_iter.peek() {
        //             if val.optional() {
        //                 continue 'schema;
        //             } else if key.as_str() != *val_key {
        //                 return Some(val_key.to_string());
        //             }
        //             res_iter.next();
        //             continue 'schema;
        //         }
        //     }
        //     is_done = true;
        //     if !val.optional() {
        //         return Some(key.to_string());
        //     }
        // }
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
