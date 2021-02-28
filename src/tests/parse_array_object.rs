use super::*;

fn run_test<'a>(schema: &'a MessyJson<'a>, value: &'a str) {
    let mut deserializer = serde_json::Deserializer::from_str(value);
    let parsed: MessyJsonValueContainer = schema.builder().deserialize(&mut deserializer).unwrap();
    assert_eq!(
        matches!(parsed.inner(), MessyJsonValue::Obj(_)),
        true,
        "The root should be an object"
    );
    match parsed.inner() {
        MessyJsonValue::Obj(obj) => {
            assert_eq!(
                obj.len(),
                1,
                "The root object should only contain a single key"
            );
            assert_eq!(
                obj.contains_key("hello"),
                true,
                "The hello key should be present"
            );
            match obj.get("hello").unwrap() {
                MessyJsonValue::Array(arr) => {
                    assert_eq!(arr.len(), 2, "The hello array should contain a 2 keys");
                    assert_eq!(
                        arr[0],
                        MessyJsonValue::String(Cow::Borrowed("the")),
                        "Value mismatch"
                    );
                    assert_eq!(
                        arr[1],
                        MessyJsonValue::String(Cow::Borrowed("world")),
                        "Value mismatch"
                    );
                }
                _ => panic!("..."),
            };
        }
        _ => panic!("..."),
    };
}

#[test]
fn simple() {
    let nested_string = MessyJson::String(Cow::Owned(MessyJsonScalar::new(false)));
    let nested_schema: MessyJson =
        MessyJson::Array(Box::new(MessyJsonArray::new(nested_string, false)));
    let schema: MessyJson = MessyJson::Obj(Cow::Owned(MessyJsonObject::new(
        vec![("hello".to_string(), nested_schema)]
            .into_iter()
            .collect(),
        false,
    )));
    let value = r#"
	{
		"hello": [
			"the",
			"world"
		]
	}
	"#;

    run_test(&schema, value);
}

#[test]
fn wrong_value() {
    let nested_string = MessyJson::String(Cow::Owned(MessyJsonScalar::new(false)));
    let nested_schema: MessyJson =
        MessyJson::Array(Box::new(MessyJsonArray::new(nested_string, false)));
    let schema: MessyJson = MessyJson::Obj(Cow::Owned(MessyJsonObject::new(
        vec![("hello".to_string(), nested_schema)]
            .into_iter()
            .collect(),
        false,
    )));
    let value = r#"
	{
		"hello": [
			1,
			2
		]
	}
	"#;

    let mut deserializer = serde_json::Deserializer::from_str(value);
    schema
        .builder()
        .deserialize(&mut deserializer)
        .expect_err("the value type should produce an error");
}
