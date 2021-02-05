use super::*;

const VAL: &str = r#"
{
	"hello": "world",
	"number": 126354,
	"bool": true,
	"array": [
		"hello",
		"hello",
		"world",
		"world"
	],
	"null": null,
	"obj": {
		"hello": "world",
		"number": 128181684654,
		"bool": true,
		"array": [
			"hello",
			"hello",
			"world",
			"world"
		],
		"null": null,
		"obj": {
			"hello": "world",
			"number": 128181684654,
			"bool": true,
			"array": [
				"hello",
				"hello",
				"world",
				"world"
			],
			"null": null
		}
	}
}
"#;

fn gen_parser() -> MessyJson {
    let schema_nested2_obj = MessyJsonObject::new(
        vec![
            (
                "hello".to_string(),
                MessyJson::String(MessyJsonScalar::new(false)),
            ),
            (
                "number".to_string(),
                MessyJson::Number(MessyJsonNumeric::new(MessyJsonNumberType::U64, false)),
            ),
            (
                "bool".to_string(),
                MessyJson::Bool(MessyJsonScalar::new(false)),
            ),
            ("null".to_string(), MessyJson::Null),
            (
                "array".to_string(),
                MessyJson::Array(Box::new(MessyJsonArray::new(
                    MessyJson::String(MessyJsonScalar::new(false)),
                    false,
                ))),
            ),
        ]
        .into_iter()
        .collect(),
        false,
    );
    let schema_nested1_obj: MessyJsonObject = MessyJsonObject::new(
        vec![
            (
                "hello".to_string(),
                MessyJson::String(MessyJsonScalar::new(false)),
            ),
            (
                "number".to_string(),
                MessyJson::Number(MessyJsonNumeric::new(MessyJsonNumberType::U64, false)),
            ),
            (
                "bool".to_string(),
                MessyJson::Bool(MessyJsonScalar::new(false)),
            ),
            ("null".to_string(), MessyJson::Null),
            (
                "array".to_string(),
                MessyJson::Array(Box::new(MessyJsonArray::new(
                    MessyJson::String(MessyJsonScalar::new(false)),
                    false,
                ))),
            ),
            (
                "obj".to_string(),
                MessyJson::Obj(Box::new(schema_nested2_obj)),
            ),
        ]
        .into_iter()
        .collect(),
        false,
    );
    let schema_root_obj: MessyJsonObject = MessyJsonObject::new(
        vec![
            (
                "hello".to_string(),
                MessyJson::String(MessyJsonScalar::new(false)),
            ),
            (
                "number".to_string(),
                MessyJson::Number(MessyJsonNumeric::new(MessyJsonNumberType::U64, false)),
            ),
            (
                "bool".to_string(),
                MessyJson::Bool(MessyJsonScalar::new(false)),
            ),
            ("null".to_string(), MessyJson::Null),
            (
                "array".to_string(),
                MessyJson::Array(Box::new(MessyJsonArray::new(
                    MessyJson::String(MessyJsonScalar::new(false)),
                    false,
                ))),
            ),
            (
                "obj".to_string(),
                MessyJson::Obj(Box::new(schema_nested1_obj)),
            ),
        ]
        .into_iter()
        .collect(),
        false,
    );
    let schema_root: MessyJson = MessyJson::Obj(Box::new(schema_root_obj));

    schema_root
}

#[test]
fn ok() {
    let parser = gen_parser();
    let mut deserializer = serde_json::Deserializer::from_str(VAL);
    let parsed_value: serde_json::Value = serde_json::from_str(VAL).unwrap();
    let parsed: MessyJsonValueContainer = parser.builder().deserialize(&mut deserializer).unwrap();

    assert_eq!(
        parsed.inner().eq(&parsed_value),
        true,
        "obj comparaison problem"
    );
}

#[test]
fn mismatch_string() {
    let parser = gen_parser();
    let mut deserializer = serde_json::Deserializer::from_str(VAL);
    let bogus_value = r#"
	{
		"hello": "worlde",
		"number": 126354,
		"bool": true,
		"array": [
			"hello",
			"hello",
			"world",
			"world"
		],
		"null": null,
		"obj": {
			"hello": "world",
			"number": 128181684654,
			"bool": true,
			"array": [
				"hello",
				"hello",
				"world",
				"world"
			],
			"null": null,
			"obj": {
				"hello": "world",
				"number": 128181684654,
				"bool": true,
				"array": [
					"hello",
					"hello",
					"world",
					"world"
				],
				"null": null
			}
		}
	}
	"#;
    let parsed_value: serde_json::Value = serde_json::from_str(bogus_value).unwrap();
    let parsed: MessyJsonValueContainer = parser.builder().deserialize(&mut deserializer).unwrap();
    assert_eq!(
        parsed.inner().eq(&parsed_value),
        false,
        "obj comparaison problem"
    );
}

#[test]
fn mismatch_number() {
    let parser = gen_parser();
    let mut deserializer = serde_json::Deserializer::from_str(VAL);
    let bogus_value = r#"
	{
		"hello": "world",
		"number": 1,
		"bool": true,
		"array": [
			"hello",
			"hello",
			"world",
			"world"
		],
		"null": null,
		"obj": {
			"hello": "world",
			"number": 128181684654,
			"bool": true,
			"array": [
				"hello",
				"hello",
				"world",
				"world"
			],
			"null": null,
			"obj": {
				"hello": "world",
				"number": 128181684654,
				"bool": true,
				"array": [
					"hello",
					"hello",
					"world",
					"world"
				],
				"null": null
			}
		}
	}
	"#;
    let parsed_value: serde_json::Value = serde_json::from_str(bogus_value).unwrap();
    let parsed: MessyJsonValueContainer = parser.builder().deserialize(&mut deserializer).unwrap();
    assert_eq!(
        parsed.inner().eq(&parsed_value),
        false,
        "obj comparaison problem"
    );
}

#[test]
fn mismatch_bool() {
    let parser = gen_parser();
    let mut deserializer = serde_json::Deserializer::from_str(VAL);
    let bogus_value = r#"
	{
		"hello": "world",
		"number": 126354,
		"bool": false,
		"array": [
			"hello",
			"hello",
			"world",
			"world"
		],
		"null": null,
		"obj": {
			"hello": "world",
			"number": 128181684654,
			"bool": true,
			"array": [
				"hello",
				"hello",
				"world",
				"world"
			],
			"null": null,
			"obj": {
				"hello": "world",
				"number": 128181684654,
				"bool": true,
				"array": [
					"hello",
					"hello",
					"world",
					"world"
				],
				"null": null
			}
		}
	}
	"#;
    let parsed_value: serde_json::Value = serde_json::from_str(bogus_value).unwrap();
    let parsed: MessyJsonValueContainer = parser.builder().deserialize(&mut deserializer).unwrap();
    assert_eq!(
        parsed.inner().eq(&parsed_value),
        false,
        "obj comparaison problem"
    );
}

#[test]
fn mismatch_array() {
    let parser = gen_parser();
    let mut deserializer = serde_json::Deserializer::from_str(VAL);
    let bogus_value = r#"
	{
		"hello": "world",
		"number": 126354,
		"bool": true,
		"array": [
			"hello",
			"helloe",
			"world",
			"world"
		],
		"null": null,
		"obj": {
			"hello": "world",
			"number": 128181684654,
			"bool": true,
			"array": [
				"hello",
				"hello",
				"world",
				"world"
			],
			"null": null,
			"obj": {
				"hello": "world",
				"number": 128181684654,
				"bool": true,
				"array": [
					"hello",
					"hello",
					"world",
					"world"
				],
				"null": null
			}
		}
	}
	"#;
    let parsed_value: serde_json::Value = serde_json::from_str(bogus_value).unwrap();
    let parsed: MessyJsonValueContainer = parser.builder().deserialize(&mut deserializer).unwrap();
    assert_eq!(
        parsed.inner().eq(&parsed_value),
        false,
        "obj comparaison problem"
    );
}

#[test]
fn mismatch_null() {
    let parser = gen_parser();
    let mut deserializer = serde_json::Deserializer::from_str(VAL);
    let bogus_value = r#"
	{
		"hello": "world",
		"number": 126354,
		"bool": true,
		"array": [
			"hello",
			"hello",
			"world",
			"world"
		],
		"null": true,
		"obj": {
			"hello": "world",
			"number": 128181684654,
			"bool": true,
			"array": [
				"hello",
				"hello",
				"world",
				"world"
			],
			"null": null,
			"obj": {
				"hello": "world",
				"number": 128181684654,
				"bool": true,
				"array": [
					"hello",
					"hello",
					"world",
					"world"
				],
				"null": null
			}
		}
	}
	"#;
    let parsed_value: serde_json::Value = serde_json::from_str(bogus_value).unwrap();
    let parsed: MessyJsonValueContainer = parser.builder().deserialize(&mut deserializer).unwrap();
    assert_eq!(
        parsed.inner().eq(&parsed_value),
        false,
        "obj comparaison problem"
    );
}

#[test]
fn mismatch_obj() {
    let parser = gen_parser();
    let mut deserializer = serde_json::Deserializer::from_str(VAL);
    let bogus_value = r#"
	{
		"hello": "world",
		"number": 126354,
		"bool": true,
		"array": [
			"hello",
			"hello",
			"world",
			"world"
		],
		"null": null,
		"obj": {
			"hello": "worlde",
			"number": 128181684654,
			"bool": true,
			"array": [
				"hello",
				"hello",
				"world",
				"world"
			],
			"null": null,
			"obj": {
				"hello": "world",
				"number": 128181684654,
				"bool": true,
				"array": [
					"hello",
					"hello",
					"world",
					"world"
				],
				"null": null
			}
		}
	}
	"#;
    let parsed_value: serde_json::Value = serde_json::from_str(bogus_value).unwrap();
    let parsed: MessyJsonValueContainer = parser.builder().deserialize(&mut deserializer).unwrap();
    assert_eq!(
        parsed.inner().eq(&parsed_value),
        false,
        "obj comparaison problem"
    );
}

#[test]
fn mismatch_obj_nested() {
    let parser = gen_parser();
    let mut deserializer = serde_json::Deserializer::from_str(VAL);
    let bogus_value = r#"
	{
		"hello": "world",
		"number": 126354,
		"bool": true,
		"array": [
			"hello",
			"hello",
			"world",
			"world"
		],
		"null": null,
		"obj": {
			"hello": "world",
			"number": 128181684654,
			"bool": true,
			"array": [
				"hello",
				"hello",
				"world",
				"world"
			],
			"null": null,
			"obj": {
				"hello": "world",
				"number": 128181684654,
				"bool": true,
				"array": [
					"hello",
					"hello",
					"worlde",
					"world"
				],
				"null": null
			}
		}
	}
	"#;
    let parsed_value: serde_json::Value = serde_json::from_str(bogus_value).unwrap();
    let parsed: MessyJsonValueContainer = parser.builder().deserialize(&mut deserializer).unwrap();
    assert_eq!(
        parsed.inner().eq(&parsed_value),
        false,
        "obj comparaison problem"
    );
}
