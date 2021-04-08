use super::*;
use crate::schema::MessyJsonObjectTrait;

fn messy_json_visit_map<'de, A, V>(
    mut seq: A,
    visitor: &V,
    obj: &MessyJsonObject,
) -> Result<MessyJsonValueContainer<'de>, A::Error>
where
    A: MapAccess<'de>,
    V: MessyJsonObjectTrait,
{
    let mut res: BTreeMap<ArcStr, MessyJsonValue> = BTreeMap::new();
    while let Some(key_seed) = seq.next_key::<&str>()? {
        let (key_str, val_schema) = obj.properties().get_key_value(key_seed).ok_or_else(|| {
            serde::de::Error::custom(format!(
                "The key `{}` is unknown. The expected keys were `[ {} ]`",
                key_seed,
                obj.properties()
                    .keys()
                    .map(|s| s.as_str())
                    .collect::<Vec<&str>>()
                    .join(", ")
            ))
        })?;
        let nested_val = visitor.new_nested(&val_schema, visitor.all_optional());
        res.insert(key_str.clone(), seq.next_value_seed(nested_val)?.take());
    }
    if !visitor.all_optional() && obj.properties().len() != res.len() {
        MessyJsonBuilder::compare_obj(obj, &mut res).map_or(Ok(()), |x| {
            Err(serde::de::Error::custom(format!("Missing key `{}`", x)))
        })?;
    }
    Ok(MessyJsonValueContainer::new(MessyJsonValue::Obj(
        MessyJsonObjectValue::from(res),
    )))
}

impl<'de> Visitor<'de> for MessyJsonBuilder {
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

        match self.inner().deref() {
            schema::MessyJsonInner::Array(arr_type) => {
                while let Some(elem) =
                    seq.next_element_seed(self.new_nested(arr_type.items(), self.all_optional()))?
                {
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
    fn visit_map<A>(self, seq: A) -> Result<Self::Value, A::Error>
    where
        A: MapAccess<'de>,
    {
        match self.inner().deref() {
            schema::MessyJsonInner::Obj(obj_type) => messy_json_visit_map(seq, &self, obj_type),
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
        match self.inner().deref() {
            schema::MessyJsonInner::Bool(_) => {
                Ok(MessyJsonValueContainer::new(MessyJsonValue::Bool(v)))
            }
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
        match self.inner().deref() {
            schema::MessyJsonInner::String(_) => Ok(MessyJsonValueContainer::new(
                MessyJsonValue::String(Cow::from(v)),
            )),
            #[cfg(feature = "uuid")]
            schema::MessyJsonInner::Uuid(_) => Ok(MessyJsonValueContainer::new(
                MessyJsonValue::Uuid(Cow::Owned(_uuid::Uuid::parse_str(v).map_err(|e| {
                    serde::de::Error::custom(format!("Failed to deserialize UUID: {}", e))
                })?)),
            )),
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
        match self.inner().deref() {
            schema::MessyJsonInner::Number(_) => Ok(MessyJsonValueContainer::new(
                MessyJsonValue::Number(v as u128),
            )),
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
        match self.inner().deref() {
            schema::MessyJsonInner::Number(_) => {
                Ok(MessyJsonValueContainer::new(MessyJsonValue::Number(v)))
            }
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
            MessyJsonNullType::Null,
            MessyJsonExpected::Root(self.inner().clone()),
        )))
    }

    #[inline]
    fn visit_some<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: Deserializer<'de>,
    {
        match self.inner().deref() {
            schema::MessyJsonInner::Bool(_) => deserializer.deserialize_bool(self),
            schema::MessyJsonInner::String(_) => deserializer.deserialize_str(self),
            schema::MessyJsonInner::Number(opt) => match opt.type_() {
                MessyJsonNumberType::U64 => deserializer.deserialize_u64(self),
                MessyJsonNumberType::U128 => deserializer.deserialize_u128(self),
            },
            schema::MessyJsonInner::Obj(_) => deserializer.deserialize_map(self),
            schema::MessyJsonInner::Array(_) => deserializer.deserialize_seq(self),
            #[cfg(feature = "uuid")]
            schema::MessyJsonInner::Uuid(_) => deserializer.deserialize_str(self),
        }
    }
}

impl<'de> Visitor<'de> for MessyJsonObjectBuilder {
    type Value = MessyJsonValueContainer<'de>;
    #[inline]
    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(formatter, "any valid json object")
    }

    #[inline]
    fn visit_some<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_map(self)
    }

    #[inline]
    fn visit_none<A>(self) -> Result<Self::Value, A>
    where
        A: serde::de::Error,
    {
        Ok(MessyJsonValueContainer::new(MessyJsonValue::Null(
            MessyJsonNullType::Absent,
            MessyJsonExpected::Obj(self.inner().clone()),
        )))
    }

    #[inline]
    fn visit_map<A>(self, seq: A) -> Result<Self::Value, A::Error>
    where
        A: MapAccess<'de>,
    {
        messy_json_visit_map(seq, &self, self.inner())
    }
}
