use super::*;
use serde::{de::DeserializeSeed, Deserialize, Serialize};
use serde_json::Value;

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

fn parse_serde_dummy_obj<T: serde::de::DeserializeOwned>(input: &str) -> T {
    serde_json::from_str(input).unwrap()
}

fn parse_serde_value_dummy_obj(input: &str) -> Value {
    serde_json::from_str(input).unwrap()
}

fn gen_messy_json_schema_dummy_obj() -> MessyJson {
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

fn parse_messy_json_dummy_obj(schema: &MessyJson) {
    let mut deserializer = serde_json::Deserializer::from_str(DUMMY_OBJ);
    let _parsed: MessyJsonValueContainer = schema.builder().deserialize(&mut deserializer).unwrap();
}

pub fn criterion_benchmark(c: &mut Criterion) {
    let prepared_dummy = gen_messy_json_schema_dummy_obj();

    let mut group = c.benchmark_group("Dummy object");
    super::apply_criterion_group_settings(&mut group);
    group.bench_with_input(
        criterion::BenchmarkId::new("deser_serde_struct", "dummy_obj"),
        &DUMMY_OBJ,
        |b, i| b.iter(|| parse_serde_dummy_obj::<DummySerdeStruct>(i)),
    );
    group.bench_with_input(
        criterion::BenchmarkId::new("deser_serde_value", "dummy_obj"),
        &DUMMY_OBJ,
        |b, i| b.iter(|| parse_serde_value_dummy_obj(i)),
    );
    group.bench_with_input(
        criterion::BenchmarkId::new("deser_messy_json", "dummy_obj"),
        &prepared_dummy,
        |b, i| b.iter(|| parse_messy_json_dummy_obj(i)),
    );
    group.finish();
}
