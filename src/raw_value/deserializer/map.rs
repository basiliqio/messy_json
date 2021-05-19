use super::*;

// This code has been **heavily** inspired by the code written in the [serde](serde) crate

struct MessyJsonRawMapKeyDeserializer<'de> {
    key: Cow<'de, str>,
}

impl<'de> Deserializer<'de> for MessyJsonRawMapKeyDeserializer<'de> {
    type Error = serde::de::value::Error;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.key {
            Cow::Owned(x) => visitor.visit_string(x),
            Cow::Borrowed(x) => visitor.visit_borrowed_str(x),
        }
    }

    serde::forward_to_deserialize_any! {
        bool i8 i16 i32 i64 i128 u8 u16 u32 u64 u128 f32 f64 char str string
        bytes byte_buf option unit unit_struct newtype_struct seq tuple
        tuple_struct map struct enum identifier ignored_any
    }
}

pub fn visit_object<'de, V>(
    object: BTreeMap<Cow<'de, str>, MessyJsonValueRaw<'de>>,
    visitor: V,
) -> Result<V::Value, serde::de::value::Error>
where
    V: Visitor<'de>,
{
    let len = object.len();
    let mut deserializer = MessyJsonRawMapDeserializer::new(object);
    let map = visitor.visit_map(&mut deserializer)?;
    let remaining = deserializer.iter.len();
    if remaining == 0 {
        Ok(map)
    } else {
        Err(serde::de::Error::invalid_length(
            len,
            &"fewer elements in map",
        ))
    }
}

struct MessyJsonRawMapDeserializer<'de> {
    iter: <BTreeMap<Cow<'de, str>, MessyJsonValueRaw<'de>> as IntoIterator>::IntoIter,
    value: Option<MessyJsonValueRaw<'de>>,
}

impl<'de> MessyJsonRawMapDeserializer<'de> {
    fn new(map: BTreeMap<Cow<'de, str>, MessyJsonValueRaw<'de>>) -> Self {
        MessyJsonRawMapDeserializer {
            iter: map.into_iter(),
            value: None,
        }
    }
}

impl<'de> MapAccess<'de> for MessyJsonRawMapDeserializer<'de> {
    type Error = serde::de::value::Error;

    fn next_key_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>, Self::Error>
    where
        T: DeserializeSeed<'de>,
    {
        match self.iter.next() {
            Some((key, value)) => {
                self.value = Some(value);
                let key_de = MessyJsonRawMapKeyDeserializer { key };
                seed.deserialize(key_de).map(Some)
            }
            None => Ok(None),
        }
    }

    fn next_value_seed<T>(&mut self, seed: T) -> Result<T::Value, Self::Error>
    where
        T: DeserializeSeed<'de>,
    {
        match self.value.take() {
            Some(value) => seed.deserialize(value),
            None => Err(serde::de::Error::custom("value is missing")),
        }
    }

    fn size_hint(&self) -> Option<usize> {
        match self.iter.size_hint() {
            (lower, Some(upper)) if lower == upper => Some(upper),
            _ => None,
        }
    }
}
