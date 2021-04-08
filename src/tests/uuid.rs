use super::*;

#[test]
fn uuid_simple() {
    let nested_string = MessyJson::from(MessyJsonInner::Uuid(MessyJsonScalar::new(false)));
    let test_uuid = _uuid::Uuid::parse_str("31ee8240-630b-416a-8c54-0e2a0d070488").unwrap();
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
		"hello": "31ee8240-630b-416a-8c54-0e2a0d070488"
	}
	"#;
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
                &test_uuid,
                "The hello key should be '{:#?}'",
                test_uuid
            );
        }
        _ => panic!("..."),
    };
}

#[test]
fn bad_uuid() {
    let nested_string = MessyJson::from(MessyJsonInner::Uuid(MessyJsonScalar::new(false)));
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
		"hello": "azaaaaaaaaaaaaaaaaa"
	}
	"#;
    let mut deserializer = serde_json::Deserializer::from_str(value);
    schema
        .builder(false)
        .deserialize(&mut deserializer)
        .unwrap_err();
}

#[test]
fn optional_uuid_present() {
    let nested_string = MessyJson::from(MessyJsonInner::Uuid(MessyJsonScalar::new(true)));
    let test_uuid = _uuid::Uuid::parse_str("31ee8240-630b-416a-8c54-0e2a0d070488").unwrap();
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
		"hello": "31ee8240-630b-416a-8c54-0e2a0d070488"
	}
	"#;
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
                &test_uuid,
                "The hello key should be '{:#?}'",
                test_uuid
            );
        }
        _ => panic!("..."),
    };
}

#[test]
fn optional_uuid_absent() {
    let nested_string = MessyJson::from(MessyJsonInner::Uuid(MessyJsonScalar::new(true)));
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
		"hello": null
	}
	"#;
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
                matches!(obj.get("hello").unwrap(), MessyJsonValue::Null(_, _)),
                true,
            );
        }
        _ => panic!("..."),
    };
}
