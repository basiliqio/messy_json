use super::*;
use serde::{de::DeserializeSeed, Deserialize, Serialize};

const DUMMY_OBJ: &str = r#"
{
	"hello": {
		"hola": "world"
	}
}
"#;

#[derive(Serialize, Deserialize)]
struct DummySerdeStructNested<'a> {
    hola: Cow<'a, str>,
}

#[derive(Serialize, Deserialize)]
struct DummySerdeStruct<'a> {
    hello: DummySerdeStructNested<'a>,
}

fn parse_serde() {
    let _parsed: DummySerdeStruct = serde_json::from_str(DUMMY_OBJ).unwrap();
}

fn gen_messy_json_schema() -> MessyJson {
    MessyJson::Obj(Box::new(MessyJsonObject::new(
        vec![(
            "hello".to_string(),
            MessyJson::Obj(Box::new(MessyJsonObject::new(
                vec![(
                    "hola".to_string(),
                    MessyJson::String(MessyJsonScalar::new(false)),
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

fn parse_messy_json(schema: &MessyJson) {
    let mut deserializer = serde_json::Deserializer::from_str(DUMMY_OBJ);
    let _parsed: MessyJsonValue = schema.deserialize(&mut deserializer).unwrap();
}

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("serde_simple_deserialize", |b| b.iter(|| parse_serde()));
    c.bench_function("messy_json_simple_deserialize_unprepared", |b| {
        b.iter(|| parse_messy_json(&gen_messy_json_schema()))
    });
    c.bench_function("messy_json_simple_deserialize_prepared", |b| {
        let prepared = gen_messy_json_schema();
        b.iter(|| parse_messy_json(&prepared))
    });
}
