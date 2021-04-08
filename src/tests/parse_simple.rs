use super::*;

fn run_flat_test<'a>(schema: &'a MessyJson<'a>, value: &'a str, expected: MessyJsonValue<'a>) {
    let mut deserializer = serde_json::Deserializer::from_str(value);
    let parsed: MessyJsonValueContainer = schema
        .builder(false)
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
    let nested_string = MessyJson::String(Cow::Owned(MessyJsonScalar::new(false)));
    let schema: MessyJson = MessyJson::Obj(Cow::Owned(MessyJsonObject::new(
        vec![(gen_key("hello"), nested_string)]
            .into_iter()
            .collect(),
        false,
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
    let nested_string = MessyJson::Bool(Cow::Owned(MessyJsonScalar::new(false)));
    let schema: MessyJson = MessyJson::Obj(Cow::Owned(MessyJsonObject::new(
        vec![(gen_key("hello"), nested_string)]
            .into_iter()
            .collect(),
        false,
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
    let nested_string = MessyJson::Number(Cow::Owned(MessyJsonNumeric::new(
        MessyJsonNumberType::U64,
        false,
    )));
    let schema: MessyJson = MessyJson::Obj(Cow::Owned(MessyJsonObject::new(
        vec![(gen_key("hello"), nested_string)]
            .into_iter()
            .collect(),
        false,
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
    let nested_string = MessyJson::Number(Cow::Owned(MessyJsonNumeric::new(
        MessyJsonNumberType::U128,
        false,
    )));
    let schema: MessyJson = MessyJson::Obj(Cow::Owned(MessyJsonObject::new(
        vec![(gen_key("hello"), nested_string)]
            .into_iter()
            .collect(),
        false,
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
