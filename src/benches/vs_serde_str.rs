use super::*;
use serde::{de::DeserializeSeed, Deserialize, Serialize};
use serde_json::Value;

const DUMMY_OBJ: &str = r#"
{
	"hello": "world"
}
"#;

#[derive(Serialize, Deserialize)]
struct DummySerdeStruct<'a> {
    hello: Cow<'a, str>,
}

fn parse_serde() {
    let _parsed: DummySerdeStruct = serde_json::from_str(DUMMY_OBJ).unwrap();
}

fn parse_serde_value() {
    let _parsed: Value = serde_json::from_str(DUMMY_OBJ).unwrap();
}

fn gen_messy_json_schema() -> MessyJson {
    MessyJson::Obj(Box::new(MessyJsonObject::new(
        vec![(
            "hello".to_string(),
            MessyJson::String(MessyJsonScalar::new(false)),
        )]
        .into_iter()
        .collect(),
        false,
    )))
}

fn parse_messy_json(schema: &MessyJson) {
    let mut deserializer = serde_json::Deserializer::from_str(DUMMY_OBJ);
    let _parsed: MessyJsonValue = schema.deserialize(&mut deserializer).unwrap();
}

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("str_serde_simple_deserialize_struct", |b| {
        b.iter(|| parse_serde())
    });
    c.bench_function("str_serde_simple_deserialize_value", |b| {
        b.iter(|| parse_serde())
    });
    // c.bench_function("messy_json_simple_deserialize_unprepared", |b| {
    //     b.iter(|| parse_messy_json(&gen_messy_json_schema()))
    // });
    c.bench_function("str_messy_json_simple_deserialize_prepared", |b| {
        let prepared = gen_messy_json_schema();
        b.iter(|| parse_messy_json(&prepared))
    });
}
