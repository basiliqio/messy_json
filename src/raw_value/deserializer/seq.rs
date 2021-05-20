use super::*;

// This code has been **heavily** inspired by the code written in the [serde](serde) crate

pub struct MessyJsonRawSeqDeserializer<'de> {
    iter: std::vec::IntoIter<MessyJsonValueRaw<'de>>,
}

impl<'de> MessyJsonRawSeqDeserializer<'de> {
    pub fn new(vec: Vec<MessyJsonValueRaw<'de>>) -> Self {
        MessyJsonRawSeqDeserializer {
            iter: vec.into_iter(),
        }
    }
}

impl<'de> SeqAccess<'de> for MessyJsonRawSeqDeserializer<'de> {
    type Error = serde::de::value::Error;

    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>, Self::Error>
    where
        T: DeserializeSeed<'de>,
    {
        match self.iter.next() {
            Some(value) => seed.deserialize(value).map(Some),
            None => Ok(None),
        }
    }

    fn size_hint(&self) -> Option<usize> {
        match self.iter.size_hint() {
            (lower, Some(upper)) if lower == upper => Some(upper),
            _ => None,
        }
    }
}

impl<'de> serde::Deserializer<'de> for MessyJsonRawSeqDeserializer<'de> {
    type Error = serde::de::value::Error;

    #[inline]
    fn deserialize_any<V>(mut self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        let len = self.iter.len();
        if len == 0 {
            visitor.visit_unit()
        } else {
            let ret = visitor.visit_seq(&mut self)?;
            let remaining = self.iter.len();
            if remaining == 0 {
                Ok(ret)
            } else {
                Err(serde::de::Error::invalid_length(
                    len,
                    &"fewer elements in array",
                ))
            }
        }
    }

    serde::forward_to_deserialize_any! {
        bool i8 i16 i32 i64 i128 u8 u16 u32 u64 u128 f32 f64 char str string
        bytes byte_buf option unit unit_struct newtype_struct seq tuple
        tuple_struct map struct enum identifier ignored_any
    }
}

pub fn visit_array<'de, V>(
    array: Vec<MessyJsonValueRaw<'de>>,
    visitor: V,
) -> Result<V::Value, serde::de::value::Error>
where
    V: Visitor<'de>,
{
    let len = array.len();
    let mut deserializer = MessyJsonRawSeqDeserializer::new(array);
    let seq = visitor.visit_seq(&mut deserializer)?;
    let remaining = deserializer.iter.len();
    if remaining == 0 {
        Ok(seq)
    } else {
        Err(serde::de::Error::invalid_length(
            len,
            &"fewer elements in array",
        ))
    }
}
