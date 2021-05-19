use super::*;

mod map;
mod seq;
use map::visit_object;
use seq::visit_array;

use std::convert::TryInto;

use serde::de::{Deserializer, Error, Visitor};

impl<'de> Deserializer<'de> for MessyJsonValueRaw<'de> {
    type Error = serde::de::value::Error;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self {
            MessyJsonValueRaw::Array(seq) => visit_array(seq, visitor),
            MessyJsonValueRaw::Obj(map) => visit_object(map, visitor),
            MessyJsonValueRaw::Number(nb) => visitor.visit_i64(nb.try_into().map_err(|_| {
                serde::de::value::Error::custom(format!("Cannot cast {} to u64", nb))
            })?),
            MessyJsonValueRaw::String(str) => match str {
                Cow::Owned(str) => visitor.visit_string(str),
                Cow::Borrowed(str) => visitor.visit_borrowed_str(str),
            },
            MessyJsonValueRaw::Null => visitor.visit_unit(),
            MessyJsonValueRaw::Bool(v) => visitor.visit_bool(v),
        }
    }

    serde::forward_to_deserialize_any! {
        bool i8 i16 i32 i64 i128 u8 u16 u32 u64 u128 f32 f64 char str string
        bytes byte_buf option unit unit_struct newtype_struct seq tuple
        tuple_struct map struct enum identifier ignored_any
    }
}
