use super::*;

// This code has been **heavily** inspired by the code written in the [serde](serde) crate

struct MessyJsonRawSeqDeserializer<'de> {
    iter: std::vec::IntoIter<MessyJsonValueRaw<'de>>,
}

impl<'de> MessyJsonRawSeqDeserializer<'de> {
    fn new(vec: Vec<MessyJsonValueRaw<'de>>) -> Self {
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
