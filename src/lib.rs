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
//! let schema: MessyJson = MessyJson::Obj(Cow::Owned(MessyJsonObject::new(
//! vec![(
//!     "hello".to_string(),
//!     MessyJson::Obj(Cow::Owned(MessyJsonObject::new(
//!         vec![(
//!             "world".to_string(),
//!             MessyJson::Number(Cow::Owned(MessyJsonNumeric::new(MessyJsonNumberType::U64, false))),
//!         )]
//!         .into_iter()
//!         .collect(),
//!         false,
//!     ))),
//! ),
//! (
//!     "an_optional_one".to_string(),
//!     MessyJson::String(Cow::Owned(MessyJsonScalar::new(true)))
//! )]
//! .into_iter()
//! .collect(),
//! false,
//! )));
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
//! # let schema: MessyJson = MessyJson::Obj(Cow::Owned(MessyJsonObject::new(
//! # vec![(
//! #     "hello".to_string(),
//! #     MessyJson::Obj(Cow::Owned(MessyJsonObject::new(
//! #         vec![(
//! #             "world".to_string(),
//! #             MessyJson::Number(Cow::Owned(MessyJsonNumeric::new(MessyJsonNumberType::U64, false))),
//! #         )]
//! #         .into_iter()
//! #         .collect(),
//! #         false,
//! #     ))),
//! # ),
//! # (
//! #     "an_optional_one".to_string(),
//! #     MessyJson::String(Cow::Owned(MessyJsonScalar::new(true)))
//! # )]
//! # .into_iter()
//! # .collect(),
//! # false,
//! # )));
//! let mut deserializer = serde_json::Deserializer::from_str(DUMMY_OBJ);
//! let val: MessyJsonValueContainer = schema.builder().deserialize(&mut deserializer).unwrap();
//!
//! println!("{:#?}", val.inner());
//! ```

#![warn(clippy::all)]
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
pub use schema::{MessyJson, MessyJsonBuilder, MessyJsonObjectBuilder};
pub use value::{
    MessyJsonArrayValue, MessyJsonNullType, MessyJsonObjectValue, MessyJsonValue,
    MessyJsonValueContainer,
};
