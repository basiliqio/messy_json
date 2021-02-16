use super::*;

impl<'de> Visitor<'de> for MessyJsonBuilder<'de> {
    type Value = MessyJsonValueContainer<'de>;
    #[inline]
    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(formatter, "any valid json object or array")
    }

    #[inline]
    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
    where
        A: SeqAccess<'de>,
    {
        let mut res: Vec<MessyJsonValue> = Vec::with_capacity(seq.size_hint().unwrap_or(0));

        match self.inner() {
            MessyJson::Array(arr_type) => {
                while let Some(elem) = seq.next_element_seed(self.new_nested(arr_type.items()))? {
                    res.push(elem.take())
                }
                Ok(MessyJsonValueContainer::new(MessyJsonValue::Array(
                    MessyJsonArrayValue::from(res),
                )))
            }
            _ => Err(serde::de::Error::invalid_type(
                serde::de::Unexpected::Seq,
                &"Sequence",
            )),
        }
    }

    #[inline]
    fn visit_map<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
    where
        A: MapAccess<'de>,
    {
        match self.inner() {
            MessyJson::Obj(obj_type) => {
                let mut res: BTreeMap<Cow<'de, str>, MessyJsonValue> = BTreeMap::new();
                while let Some(key_seed) = seq.next_key_seed(
                    self.new_nested(&MessyJson::String(MessyJsonScalar { optional: false })),
                )? {
                    let (val_schema, key_str) = match key_seed.take() {
                        MessyJsonValue::String(val) => (
                            obj_type.properties().get(&*val).ok_or_else(|| {
                                serde::de::Error::custom(format!(
                                    "The key `{}` is unknown. The expected keys were `[ {} ]`",
                                    val,
                                    obj_type
                                        .properties()
                                        .keys()
                                        .map(|s| s.as_str())
                                        .collect::<Vec<&str>>()
                                        .join(", ")
                                ))
                            })?,
                            val,
                        ),
                        _ => {
                            return Err(serde::de::Error::invalid_type(
                                serde::de::Unexpected::Map,
                                &"String",
                            ));
                        }
                    };
                    let nested_val = self.new_nested(&val_schema);
                    res.insert(key_str, seq.next_value_seed(nested_val)?.take());
                }
                if obj_type.properties().len() != res.len() {
                    Self::compare_obj(obj_type, &res).map_or(Ok(()), |x| {
                        Err(serde::de::Error::custom(format!("Missing key `{}`", x)))
                    })?;
                }
                Ok(MessyJsonValueContainer::new(MessyJsonValue::Obj(
                    MessyJsonObjectValue::from(res),
                )))
            }
            _ => Err(serde::de::Error::invalid_type(
                serde::de::Unexpected::Map,
                &"Map",
            )),
        }
    }

    #[inline]
    fn visit_bool<A>(self, v: bool) -> Result<Self::Value, A>
    where
        A: serde::de::Error,
    {
        match self.inner() {
            MessyJson::Bool(_) => Ok(MessyJsonValueContainer::new(MessyJsonValue::Bool(v))),
            _ => Err(serde::de::Error::invalid_type(
                serde::de::Unexpected::Bool(v),
                &"other",
            )),
        }
    }

    #[inline]
    fn visit_borrowed_str<A>(self, v: &'de str) -> Result<Self::Value, A>
    where
        A: serde::de::Error,
    {
        match self.inner() {
            MessyJson::String(_) => Ok(MessyJsonValueContainer::new(MessyJsonValue::String(
                Cow::from(v),
            ))),
            _ => Err(serde::de::Error::invalid_type(
                serde::de::Unexpected::Str(v),
                &"String",
            )),
        }
    }

    #[inline]
    fn visit_u64<A>(self, v: u64) -> Result<Self::Value, A>
    where
        A: serde::de::Error,
    {
        match self.inner() {
            MessyJson::Number(_) => Ok(MessyJsonValueContainer::new(MessyJsonValue::Number(
                v as u128,
            ))),
            _ => Err(serde::de::Error::invalid_type(
                serde::de::Unexpected::Other("number"),
                &"Number",
            )),
        }
    }

    #[inline]
    fn visit_u128<A>(self, v: u128) -> Result<Self::Value, A>
    where
        A: serde::de::Error,
    {
        match self.inner() {
            MessyJson::Number(_) => Ok(MessyJsonValueContainer::new(MessyJsonValue::Number(v))),
            _ => Err(serde::de::Error::invalid_type(
                serde::de::Unexpected::Other("number"),
                &"Number",
            )),
        }
    }

    #[inline]
    fn visit_none<A>(self) -> Result<Self::Value, A>
    where
        A: serde::de::Error,
    {
        Ok(MessyJsonValueContainer::new(MessyJsonValue::Null(
            self.inner(),
        )))
    }

    #[inline]
    fn visit_some<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: Deserializer<'de>,
    {
        match self.inner() {
            MessyJson::Bool(_) => deserializer.deserialize_bool(self),
            MessyJson::String(_) => deserializer.deserialize_str(self),
            MessyJson::Number(opt) => match opt.type_() {
                MessyJsonNumberType::U64 => deserializer.deserialize_u64(self),
                MessyJsonNumberType::U128 => deserializer.deserialize_u128(self),
            },
            MessyJson::Obj(_) => deserializer.deserialize_map(self),
            MessyJson::Array(_) => deserializer.deserialize_seq(self),
        }
    }
}
