use super::*;

impl<'de> Visitor<'de> for &'de MessyJson {
    type Value = MessyJsonValue<'de>;
    #[inline]
    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(formatter, "any valid json object or array")
    }

    #[inline]
    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
    where
        A: SeqAccess<'de>,
    {
        let mut res: Vec<Self::Value> = Vec::with_capacity(seq.size_hint().unwrap_or(0));

        match self {
            MessyJson::Array(arr_type) => {
                while let Some(elem) = seq.next_element_seed(arr_type.items())? {
                    res.push(elem)
                }
                Ok(MessyJsonValue::Array(res))
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
        match self {
            MessyJson::Obj(obj_type) => {
                let mut res: BTreeMap<Cow<'de, str>, Self::Value> = BTreeMap::new();
                while let Some(key_seed) =
                    seq.next_key_seed(&MessyJson::String(MessyJsonScalar { optional: false }))?
                {
                    let (val_schema, key_str) = match key_seed {
                        MessyJsonValue::String(val) => (
                            obj_type.properties().get(&*val).ok_or_else(|| {
                                serde::de::Error::custom(format!(
                                    "The key `{}` is unknown. The expected keys were [{}]",
                                    val,
                                    obj_type
                                        .properties()
                                        .keys()
                                        .filter_map(|s| match res.contains_key(s.as_str()) {
                                            false => Some(s.as_str()),
                                            true => None,
                                        })
                                        .collect::<Vec<&str>>()
                                        .join(",")
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
                    res.insert(key_str, seq.next_value_seed(val_schema)?);
                }
                Ok(MessyJsonValue::Obj(res))
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
        match self {
            MessyJson::Bool(_) => Ok(MessyJsonValue::Bool(v)),
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
        match self {
            MessyJson::String(_) => Ok(MessyJsonValue::String(Cow::from(v))),
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
        match self {
            MessyJson::Number(_) => Ok(MessyJsonValue::Number(v as u128)),
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
        match self {
            MessyJson::Number(_) => Ok(MessyJsonValue::Number(v)),
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
        match self {
            MessyJson::Null => Ok(MessyJsonValue::Null),
            _ => Err(serde::de::Error::invalid_type(
                serde::de::Unexpected::Other("null"),
                &"Null",
            )),
        }
    }

    #[inline]
    fn visit_some<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: Deserializer<'de>,
    {
        match self {
            MessyJson::Bool(_) => deserializer.deserialize_bool(self),
            MessyJson::String(_) => deserializer.deserialize_str(self),
            MessyJson::Number(opt) => match opt.type_() {
                MessyJsonNumberType::U64 => deserializer.deserialize_u64(self),
                MessyJsonNumberType::U128 => deserializer.deserialize_u128(self),
            },
            MessyJson::Obj(_) => deserializer.deserialize_map(self),
            MessyJson::Array(_) => deserializer.deserialize_seq(self),
            MessyJson::Null => deserializer.deserialize_option(self),
        }
    }
}
