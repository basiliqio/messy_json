use super::*;

#[test]
fn simple() {
    let nested_string = MessyJson::from(MessyJsonInner::String(MessyJsonScalar::new(false)));
    let schema = MessyJson::from(MessyJsonInner::Array(MessyJsonArray::new(
        nested_string,
        false,
    )));
    let value = r#"
	[
		"hello",
		"world"
	]
	"#;

    let mut deserializer = serde_json::Deserializer::from_str(value);
    let parsed: MessyJsonValueContainer = schema
        .builder(false)
        .deserialize(&mut deserializer)
        .unwrap();
    assert_eq!(
        matches!(parsed.inner(), MessyJsonValue::Array(_)),
        true,
        "The root should be an array"
    );
    match parsed.inner() {
        MessyJsonValue::Array(arr) => {
            assert_eq!(arr.len(), 2, "The root object should contain 2 keys");
            assert_eq!(
                arr[0],
                MessyJsonValue::String(Cow::Borrowed("hello")),
                "Values mismatch"
            );
            assert_eq!(
                arr[1],
                MessyJsonValue::String(Cow::Borrowed("world")),
                "Values mismatch"
            );
        }
        _ => panic!("..."),
    };
}
