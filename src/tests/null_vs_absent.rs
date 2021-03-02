use super::*;

#[test]
fn null() {
    let nested_string = MessyJson::String(Cow::Owned(MessyJsonScalar::new(true)));
    let schema: MessyJsonObject = MessyJsonObject::new(
        vec![("hello".to_string(), nested_string)]
            .into_iter()
            .collect(),
        false,
    );
    let value = r#"
	{
		"hello": null
	}
	"#;

    let mut deserializer = serde_json::Deserializer::from_str(value);
    let parsed = schema.builder().deserialize(&mut deserializer).unwrap();
    match parsed.inner() {
        MessyJsonValue::Obj(x) => assert_eq!(
            matches!(x.get("hello").unwrap(), MessyJsonValue::Null(x, _y) if matches!(x, MessyJsonNullType::Null)),
            true
        ),
        _ => panic!(),
    }
}

#[test]
fn absent() {
    let nested_string = MessyJson::String(Cow::Owned(MessyJsonScalar::new(true)));
    let schema: MessyJsonObject = MessyJsonObject::new(
        vec![("hello".to_string(), nested_string)]
            .into_iter()
            .collect(),
        false,
    );
    let value = r#"
	{
	}
	"#;

    let mut deserializer = serde_json::Deserializer::from_str(value);
    let parsed = schema.builder().deserialize(&mut deserializer).unwrap();
    match parsed.inner() {
        MessyJsonValue::Obj(x) => assert_eq!(
            matches!(x.get("hello").unwrap(), MessyJsonValue::Null(x, _y) if matches!(x, MessyJsonNullType::Absent)),
            true
        ),
        _ => panic!(),
    }
}
