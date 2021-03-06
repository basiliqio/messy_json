use super::*;
use serde::de::DeserializeSeed;

mod all_optional;
mod cmp_value;
mod null_vs_absent;
mod parse_array_object;
mod parse_nested_object;
mod parse_simple;
mod root_array;
mod unexact_obj;

#[cfg(feature = "uuid")]
mod uuid;
