use super::*;

#[test]
fn all_absent() {
    let nested_string = MessyJson::from(MessyJsonInner::String(MessyJsonScalar::new(false)));
    let nested_schema: MessyJson = MessyJson::from(MessyJsonInner::Obj(MessyJsonObject::from(
        MessyJsonObjectInner::new(
            vec![(gen_key("the"), nested_string)].into_iter().collect(),
            false,
        ),
    )));
    let schema: MessyJson = MessyJson::from(MessyJsonInner::Obj(MessyJsonObject::from(
        MessyJsonObjectInner::new(
            vec![(gen_key("hello"), nested_schema)]
                .into_iter()
                .collect(),
            false,
        ),
    )));
    let value = r#"
	{
	}
	"#;
    let mut deserializer = serde_json::Deserializer::from_str(value);
    let parsed: MessyJsonValueContainer = schema
        .builder(MessyJsonSettings {
            all_optional: true,
            preserve_mandatory: false,
        })
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
                0,
                "The root object shouldn't contain a single key"
            );
        }
        _ => panic!("..."),
    };
}

#[test]
fn mix_absent() {
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
		"hello":
		{

		}
	}
	"#;
    let mut deserializer = serde_json::Deserializer::from_str(value);
    let parsed: MessyJsonValueContainer = schema
        .builder(MessyJsonSettings {
            all_optional: true,
            preserve_mandatory: false,
        })
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
                "The hello key shouldn't be present"
            );
            match obj.get("hello").unwrap() {
                MessyJsonValue::Obj(obj) => {
                    assert_eq!(
                        obj.len(),
                        0,
                        "The root object shouldn't contain a single key"
                    );
                }
                _ => panic!("..."),
            };
        }
        _ => panic!("..."),
    };
}

#[test]
fn all_present() {
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
		"hello":
		{
			"the": "world"
		}
	}
	"#;
    let mut deserializer = serde_json::Deserializer::from_str(value);
    let parsed: MessyJsonValueContainer = schema
        .builder(MessyJsonSettings {
            all_optional: true,
            preserve_mandatory: false,
        })
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
                "The hello key shouldn't be present"
            );
            match obj.get("hello").unwrap() {
                MessyJsonValue::Obj(obj) => {
                    assert_eq!(
                        obj.len(),
                        1,
                        "The root object shouldn't contain a single key"
                    );
                    assert_eq!(
                        obj.contains_key("the"),
                        true,
                        "The hello key should be present"
                    );
                    assert_eq!(
                        obj.get("the").unwrap(),
                        &MessyJsonValue::String(Cow::Borrowed("world")),
                        "The 'the' key should be '{:#?}'",
                        "world"
                    );
                }
                _ => panic!("..."),
            };
        }
        _ => panic!("..."),
    };
}

#[test]
fn unkown_keys() {
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
		"coucou": "wassup"
	}
	"#;
    let mut deserializer = serde_json::Deserializer::from_str(value);
    schema
        .builder(MessyJsonSettings {
            all_optional: true,
            preserve_mandatory: false,
        })
        .deserialize(&mut deserializer)
        .unwrap_err();
}
