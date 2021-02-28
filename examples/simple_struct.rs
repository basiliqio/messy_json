extern crate messy_json;
use messy_json::*;
use serde::{de::DeserializeSeed, Deserialize, Serialize};
use serde_json::Value;
use std::borrow::Cow;

const DUMMY_OBJ: &str = r#"
{
	"hello": {
		"hola": "world"
	}
}
"#;

#[derive(Debug, Serialize, Deserialize)]
struct DummySerdeStructNested<'a> {
    hola: Cow<'a, str>,
}

#[derive(Debug, Serialize, Deserialize)]
struct DummySerdeStruct<'a> {
    hello: DummySerdeStructNested<'a>,
}

fn parse_serde_value() -> Value {
    serde_json::from_str(DUMMY_OBJ).unwrap()
}

fn parse_serde_struct() -> DummySerdeStruct<'static> {
    serde_json::from_str(DUMMY_OBJ).unwrap()
}

fn gen_messy_json_schema<'a>() -> MessyJson<'a> {
    MessyJson::Obj(Cow::Owned(MessyJsonObject::new(
        vec![(
            "hello".to_string(),
            MessyJson::Obj(Cow::Owned(MessyJsonObject::new(
                vec![(
                    "hola".to_string(),
                    MessyJson::String(Cow::Owned(MessyJsonScalar::new(false))),
                )]
                .into_iter()
                .collect(),
                false,
            ))),
        )]
        .into_iter()
        .collect(),
        false,
    )))
}

fn parse_messy_json<'a>(schema: &'a MessyJson<'a>) -> MessyJsonValueContainer<'a> {
    let mut deserializer = serde_json::Deserializer::from_str(DUMMY_OBJ);
    schema.builder().deserialize(&mut deserializer).unwrap()
}

fn main() {
    let prepared = gen_messy_json_schema();

    println!("Struct : {:#?}", parse_serde_struct());
    println!("Value : {:#?}", parse_serde_value());
    println!("MessyJsonValue : {:#?}", parse_messy_json(&prepared));
}
