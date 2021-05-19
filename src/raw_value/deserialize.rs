use super::*;

#[derive(Clone, Copy, Default)]
pub struct MessyJsonValueRawVisitor;

impl<'de> serde::de::Visitor<'de> for MessyJsonValueRawVisitor {
    type Value = MessyJsonValueRaw<'de>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(formatter, "any valid json")
    }
    fn visit_bool<E>(self, v: bool) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(MessyJsonValueRaw::Bool(v))
    }

    fn visit_borrowed_str<E>(self, v: &'de str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(MessyJsonValueRaw::String(v.into()))
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(MessyJsonValueRaw::String(v.to_string().into()))
    }

    fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(MessyJsonValueRaw::String(v.into()))
    }

    fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(MessyJsonValueRaw::from(v))
    }

    fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(MessyJsonValueRaw::from(v))
    }

    fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(MessyJsonValueRaw::from(v))
    }

    fn visit_unit<E>(self) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(MessyJsonValueRaw::Null)
    }

    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: MapAccess<'de>,
    {
        let mut res: BTreeMap<Cow<'de, str>, MessyJsonValueRaw<'de>> = BTreeMap::new();

        while let Some(el) = map.next_entry()? {
            res.insert(el.0, el.1);
        }
        Ok(MessyJsonValueRaw::Obj(res))
    }

    #[inline]
    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
    where
        A: SeqAccess<'de>,
    {
        let mut res: Vec<MessyJsonValueRaw<'de>> =
            Vec::with_capacity(seq.size_hint().unwrap_or_default());

        while let Some(next) = seq.next_element()? {
            res.push(next);
        }
        Ok(MessyJsonValueRaw::Array(res))
    }
}

impl<'de> serde::Deserialize<'de> for MessyJsonValueRaw<'de> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_any(MessyJsonValueRawVisitor::default())
    }
}
