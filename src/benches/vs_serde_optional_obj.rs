use super::*;
use serde::{de::DeserializeSeed, Deserialize, Serialize};
use serde_json::Value;

const OPTIONAL_OBJ: &str = r#"
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
    coucou: Option<Cow<'a, str>>,
    coucou1: Option<Cow<'a, str>>,
    coucou2: Option<Cow<'a, str>>,
}

fn parse_serde_optional_obj<T: serde::de::DeserializeOwned>(input: &str) -> T {
    serde_json::from_str(input).unwrap()
}

fn parse_serde_value_optional_obj(input: &str) -> Value {
    serde_json::from_str(input).unwrap()
}

fn gen_messy_json_schema_optional_obj<'a>() -> MessyJson<'a> {
    MessyJson::Obj(Cow::Owned(MessyJsonObject::new(
        vec![
            (
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
            ),
            (
                "coucou".to_string(),
                MessyJson::String(Cow::Owned(MessyJsonScalar::new(true))),
            ),
            (
                "coucou1".to_string(),
                MessyJson::String(Cow::Owned(MessyJsonScalar::new(true))),
            ),
            (
                "coucou2".to_string(),
                MessyJson::String(Cow::Owned(MessyJsonScalar::new(true))),
            ),
        ]
        .into_iter()
        .collect(),
        false,
    )))
}

fn parse_messy_json_optional_obj<'a>(schema: &'a MessyJson<'a>) {
    let mut deserializer = serde_json::Deserializer::from_str(OPTIONAL_OBJ);
    let _parsed: MessyJsonValueContainer = schema
        .builder(false)
        .deserialize(&mut deserializer)
        .unwrap();
}

pub fn criterion_benchmark(c: &mut Criterion) {
    let prepared_optional = gen_messy_json_schema_optional_obj();

    let mut group = c.benchmark_group("Partial object");
    super::apply_criterion_group_settings(&mut group);
    group.bench_with_input(
        criterion::BenchmarkId::new("deser_serde_struct", "optional_obj"),
        &OPTIONAL_OBJ,
        |b, i| b.iter(|| parse_serde_optional_obj::<DummySerdeStruct>(i)),
    );
    group.bench_with_input(
        criterion::BenchmarkId::new("deser_serde_value", "optional_obj"),
        &OPTIONAL_OBJ,
        |b, i| b.iter(|| parse_serde_value_optional_obj(i)),
    );
    group.bench_with_input(
        criterion::BenchmarkId::new("deser_messy_json", "optional_obj"),
        &prepared_optional,
        |b, _i| b.iter(|| parse_messy_json_optional_obj(&prepared_optional)),
    );
    group.finish()
}
