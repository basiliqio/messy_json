use super::*;

fn run_flat_test(schema: &MessyJson, value: &str, expected: MessyJsonValue) {
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
            assert_eq!(
                obj.get("hello").unwrap(),
                &expected,
                "The hello key should be '{:#?}'",
                expected
            );
        }
        _ => panic!("..."),
    };
}

#[test]
fn string() {
    let nested_string = MessyJson::from(MessyJsonInner::String(MessyJsonScalar::new(false)));
    let schema = MessyJson::from(MessyJsonInner::Obj(MessyJsonObject::from(
        MessyJsonObjectInner::new(
            vec![(gen_key("hello"), nested_string)]
                .into_iter()
                .collect(),
            false,
        ),
    )));
    let value = r#"
	{
		"hello": "world"
	}
	"#;

    run_flat_test(
        &schema,
        value,
        MessyJsonValue::String(Cow::Borrowed("world")),
    );
}

#[test]
fn bool() {
    let nested_string = MessyJson::from(MessyJsonInner::Bool(MessyJsonScalar::new(false)));
    let schema = MessyJson::from(MessyJsonInner::Obj(MessyJsonObject::from(
        MessyJsonObjectInner::new(
            vec![(gen_key("hello"), nested_string)]
                .into_iter()
                .collect(),
            false,
        ),
    )));
    let value = r#"
	{
		"hello": true
	}
	"#;
    run_flat_test(&schema, value, MessyJsonValue::Bool(true));
}

#[test]
fn number_tiny() {
    let nested_string = MessyJson::from(MessyJsonInner::Number(MessyJsonNumeric::new(
        MessyJsonNumberType::U64,
        false,
    )));
    let schema = MessyJson::from(MessyJsonInner::Obj(MessyJsonObject::from(
        MessyJsonObjectInner::new(
            vec![(gen_key("hello"), nested_string)]
                .into_iter()
                .collect(),
            false,
        ),
    )));
    let value = r#"
	{
		"hello": 15
	}
	"#;
    run_flat_test(&schema, value, MessyJsonValue::Number(15));
}

#[test]
fn number_huge() {
    let nested_string = MessyJson::from(MessyJsonInner::Number(MessyJsonNumeric::new(
        MessyJsonNumberType::U128,
        false,
    )));
    let schema = MessyJson::from(MessyJsonInner::Obj(MessyJsonObject::from(
        MessyJsonObjectInner::new(
            vec![(gen_key("hello"), nested_string)]
                .into_iter()
                .collect(),
            false,
        ),
    )));
    let value = r#"
	{
		"hello": 340282366920938463463374607431768211454
	}
	"#;
    run_flat_test(
        &schema,
        value,
        MessyJsonValue::Number(340282366920938463463374607431768211454),
    );
}
