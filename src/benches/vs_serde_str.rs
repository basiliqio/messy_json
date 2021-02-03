use super::*;
use serde::{de::DeserializeSeed, Deserialize, Serialize};
use serde_json::Value;

const SIMPLE_OBJ: &str = r#"
{
	"hello": "world"
}
"#;

#[derive(Serialize, Deserialize)]
struct SimpleStruct<'a> {
    hello: Cow<'a, str>,
}

fn parse_serde(input: &str) -> SimpleStruct {
    serde_json::from_str(input).unwrap()
}

fn parse_serde_value(input: &str) -> Value {
    serde_json::from_str(input).unwrap()
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

fn parse_messy_json(schema: &MessyJson, input: &str) {
    let mut deserializer = serde_json::Deserializer::from_str(input);
    let _parsed: MessyJsonValueContainer = schema.builder().deserialize(&mut deserializer).unwrap();
}

pub fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("Simple object");

    super::apply_criterion_group_settings(&mut group);
    group.bench_with_input(
        criterion::BenchmarkId::new("deser_serde_struct", "simple_obj"),
        &SIMPLE_OBJ,
        |b, i| b.iter(|| parse_serde(i)),
    );
    group.bench_with_input(
        criterion::BenchmarkId::new("deser_serde_value", "simple_obj"),
        &SIMPLE_OBJ,
        |b, i| b.iter(|| parse_serde_value(i)),
    );
    group.bench_with_input(
        criterion::BenchmarkId::new("deser_messy_json", "simple_obj"),
        &SIMPLE_OBJ,
        |b, i| {
            let prepared = gen_messy_json_schema();
            b.iter(|| parse_messy_json(&prepared, i))
        },
    );
    group.finish();
}
