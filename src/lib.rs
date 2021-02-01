use serde::de::{DeserializeSeed, Deserializer, MapAccess, SeqAccess, Visitor};
use std::borrow::Cow;
use std::collections::BTreeMap;

mod array;
mod number;
mod object;
mod scalar;
mod schema;
mod schema_visitor;
mod value;

#[cfg(test)]
mod tests;

pub use array::MessyJsonArray;
pub use number::{MessyJsonNumberType, MessyJsonNumeric};
pub use object::MessyJsonObject;
pub use scalar::MessyJsonScalar;
pub use schema::MessyJson;
pub use value::MessyJsonValue;
