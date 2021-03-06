use super::*;

fn run_test(schema: &MessyJson, value: &str, expected: MessyJsonValue) {
    let mut deserializer = serde_json::Deserializer::from_str(value);
    let parsed: MessyJsonValueContainer = schema
        .builder(MessyJsonSettings::default())
        .deserialize(&mut deserializer)
        .unwrap();
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
                MessyJsonValue::Obj(obj) => {
                    assert_eq!(
                        obj.len(),
                        1,
                        "The root object should only contain a single key"
                    );
                    assert_eq!(
                        obj.contains_key("the"),
                        true,
                        "The hello key should be present"
                    );
                    assert_eq!(
                        obj.get("the").unwrap(),
                        &expected,
                        "The 'the' key should be '{:#?}'",
                        expected
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
    let nested_string = MessyJson::from(MessyJsonInner::String(MessyJsonScalar::new(false)));
    let nested_schema = MessyJson::from(MessyJsonInner::Obj(MessyJsonObject::from(
        MessyJsonObjectInner::new(
            vec![(gen_key("the"), nested_string)].into_iter().collect(),
            false,
        ),
    )));
    let schema = MessyJson::from(MessyJsonInner::Obj(MessyJsonObject::from(
        MessyJsonObjectInner::new(
            vec![(gen_key("hello"), nested_schema)]
                .into_iter()
                .collect(),
            false,
        ),
    )));
    let value = r#"
	{
		"hello": {
			"the": "world"
		}
	}
	"#;

    run_test(
        &schema,
        value,
        MessyJsonValue::String(Cow::Borrowed("world")),
    );
}

#[test]
fn wrong_key() {
    let nested_string = MessyJson::from(MessyJsonInner::String(MessyJsonScalar::new(false)));
    let nested_schema = MessyJson::from(MessyJsonInner::Obj(MessyJsonObject::from(
        MessyJsonObjectInner::new(
            vec![(gen_key("the"), nested_string)].into_iter().collect(),
            false,
        ),
    )));
    let schema = MessyJson::from(MessyJsonInner::Obj(MessyJsonObject::from(
        MessyJsonObjectInner::new(
            vec![(gen_key("hello"), nested_schema)]
                .into_iter()
                .collect(),
            false,
        ),
    )));
    let value = r#"
	{
		"hello": {
			"hola": "world"
		}
	}
	"#;

    let mut deserializer = serde_json::Deserializer::from_str(value);
    schema
        .builder(MessyJsonSettings::default())
        .deserialize(&mut deserializer)
        .expect_err("to fail because of wrong key");
}

#[test]
fn wrong_value_type() {
    let nested_string = MessyJson::from(MessyJsonInner::String(MessyJsonScalar::new(false)));
    let nested_schema = MessyJson::from(MessyJsonInner::Obj(MessyJsonObject::from(
        MessyJsonObjectInner::new(
            vec![(gen_key("the"), nested_string)].into_iter().collect(),
            false,
        ),
    )));
    let schema = MessyJson::from(MessyJsonInner::Obj(MessyJsonObject::from(
        MessyJsonObjectInner::new(
            vec![(gen_key("hello"), nested_schema)]
                .into_iter()
                .collect(),
            false,
        ),
    )));
    let value = r#"
	{
		"hello": {
			"the": 61
		}
	}
	"#;

    let mut deserializer = serde_json::Deserializer::from_str(value);
    schema
        .builder(MessyJsonSettings::default())
        .deserialize(&mut deserializer)
        .expect_err("to fail because of wrong value");
}
