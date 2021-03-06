//! ## Introduction
//!
//! The rust ecosystem allows for **very** good compile-time implementation of JSON deserializer to rust structure, however,
//! things get a bit more sparse when it come to run-time deserialization of dynamically structured objects.
//!
//! This crate approaches this problems in a simple manner, resembling [`serde_json`'s `Value`](serde_json::Value).
//!
//!
//! ## Usage
//!
//! When deserializing from a known structure type, one would just call the `serde`'s deserializer and let it do its magic.
//! However in this case, one needs to define how the JSON will be structured.
//!
//! ### Defining the schema
//!
//! To do that, one can use the provided object : [MessyJson](MessyJson)
//!
//! For instance defining an object that could look like the following in JSON :
//!
//! ```json
//! {
//!     "hello": {
//!         "world": 128
//!     },
//!     "an_optional_one": "Waou"
//! }
//! ```
//!
//! One would define the following [MessyJson](MessyJson) :
//!
//! ```rust
//! # use messy_json::*;
//! # use std::borrow::Cow;
//!
//! let schema = MessyJson::from(MessyJsonInner::Obj(MessyJsonObject::from(MessyJsonObjectInner::new(
//!    vec![(
//!        arcstr::literal!("hello"),
//!        MessyJson::from(MessyJsonInner::Obj(MessyJsonObject::from(MessyJsonObjectInner::new(
//!            vec![(
//!                arcstr::literal!("world"),
//!                MessyJson::from(MessyJsonInner::String(MessyJsonScalar::new(false))),
//!            )]
//!            .into_iter()
//!            .collect(),
//!            false,
//!        )))),
//!    ),
//! (
//!     arcstr::literal!("an_optional_one"),
//!     MessyJson::from(MessyJsonInner::String(MessyJsonScalar::new(true)))
//! )]
//!    .into_iter()
//!    .collect(),
//!    false,
//! ))));
//! ```
//!
//! Granted, this is a bit _wordy_ to define such a simple structure but keep in
//! mind that this should'nt be hand-written and should be composed by your application logic.
//!
//! ### Parsing the schema
//!
//! To parse the `&str` using the schema one only need to crweate the deserializer
//! and call it using the schema [builder](MessyJsonBuilder) :
//!
//! ```rust
//! # use serde::de::DeserializeSeed;
//! # use messy_json::*;
//! # use std::borrow::Cow;
//!
//! const DUMMY_OBJ: &str = r#"
//! {
//!         "hello": {
//!             "world": 128
//!         },
//!         "an_optional_one": "Waou"
//!     }
//! "#;
//!
//! # let schema = MessyJson::from(MessyJsonInner::Obj(MessyJsonObject::from(MessyJsonObjectInner::new(
//! #    vec![(
//! #        arcstr::literal!("hello"),
//! #        MessyJson::from(MessyJsonInner::Obj(MessyJsonObject::from(MessyJsonObjectInner::new(
//! #            vec![(
//! #                arcstr::literal!("world"),
//! #                MessyJson::from(MessyJsonInner::Number(MessyJsonNumeric::new(MessyJsonNumberType::U64, false))),
//! #            )]
//! #            .into_iter()
//! #            .collect(),
//! #            false,
//! #        )))),
//! #    ),
//! # (
//! #     arcstr::literal!("an_optional_one"),
//! #     MessyJson::from(MessyJsonInner::String(MessyJsonScalar::new(true)))
//! # )]
//! #    .into_iter()
//! #    .collect(),
//! #    false,
//! # ))));
//! let mut deserializer = serde_json::Deserializer::from_str(DUMMY_OBJ);
//! let val: MessyJsonValueContainer = schema.builder(MessyJsonSettings::default()).deserialize(&mut deserializer).unwrap();
//!
//! println!("{:#?}", val.inner());
//! ```
#![warn(clippy::all)]

#[cfg(doctest)]
doc_comment::doctest!("../README.md");

use arcstr::ArcStr;
use serde::de::{DeserializeSeed, Deserializer, MapAccess, SeqAccess, Visitor};
use std::borrow::Cow;
use std::collections::BTreeMap;
use std::ops::Deref;
use std::sync::Arc;

mod array;
mod number;
mod object;
mod raw_value;
mod scalar;
mod schema;
mod schema_visitor;
mod settings;
mod value;

#[cfg(test)]
mod tests;

pub use array::MessyJsonArray;
pub use number::{MessyJsonNumberType, MessyJsonNumeric};
#[cfg(test)]
pub use object::gen_key;
pub use object::{KeyType, MessyJsonObject, MessyJsonObjectInner};
pub use raw_value::{MessyJsonValueRaw, MessyJsonValueRawVisitor};
pub use scalar::MessyJsonScalar;
pub use schema::{
    MessyJson, MessyJsonBuilder, MessyJsonExpected, MessyJsonInner, MessyJsonObjectBuilder,
};
pub use settings::MessyJsonSettings;
pub use value::{
    MessyJsonArrayValue, MessyJsonNullType, MessyJsonObjectValue, MessyJsonValue,
    MessyJsonValueContainer,
};
