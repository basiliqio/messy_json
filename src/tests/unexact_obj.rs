use super::*;

#[test]
fn unknown() {
    let nested_string = MessyJson::String(MessyJsonScalar::new(false));
    let schema: MessyJson = MessyJson::Obj(MessyJsonObject::new(
        vec![("hello".to_string(), nested_string)]
            .into_iter()
            .collect(),
        false,
    ));
    let value = r#"
	{
		"hello": "world",
		"whoami": "wellidk"
	}
	"#;

    let mut deserializer = serde_json::Deserializer::from_str(value);
    let parsed = schema.builder().deserialize(&mut deserializer).unwrap_err();
    println!("{:#?}", parsed);
}

#[test]
fn missing() {
    let nested_string = MessyJson::String(MessyJsonScalar::new(false));
    let schema: MessyJson = MessyJson::Obj(MessyJsonObject::new(
        vec![("hello".to_string(), nested_string)]
            .into_iter()
            .collect(),
        false,
    ));
    let value = r#"
	{
	}
	"#;

    let mut deserializer = serde_json::Deserializer::from_str(value);
    let parsed = schema.builder().deserialize(&mut deserializer).unwrap_err();
    println!("{:#?}", parsed);
}

#[test]
fn complete_with_optional() {
    let nested_string = MessyJson::String(MessyJsonScalar::new(false));
    let nested_string_opt = MessyJson::String(MessyJsonScalar::new(true));
    let schema: MessyJson = MessyJson::Obj(MessyJsonObject::new(
        vec![
            ("hello".to_string(), nested_string.clone()),
            ("whoami".to_string(), nested_string_opt),
            ("hehe".to_string(), nested_string),
        ]
        .into_iter()
        .collect(),
        false,
    ));
    let value = r#"
	{
		"hello": "world",
		"whoami": "wellidk",
		"hehe": "hoho"
	}
	"#;

    let mut deserializer = serde_json::Deserializer::from_str(value);
    schema.builder().deserialize(&mut deserializer).unwrap();
}

#[test]
fn incomplete_with_optional() {
    let nested_string = MessyJson::String(MessyJsonScalar::new(false));
    let nested_string_opt = MessyJson::String(MessyJsonScalar::new(true));
    let schema: MessyJson = MessyJson::Obj(MessyJsonObject::new(
        vec![
            ("hello".to_string(), nested_string.clone()),
            ("whoami".to_string(), nested_string_opt),
            ("hehe".to_string(), nested_string),
        ]
        .into_iter()
        .collect(),
        false,
    ));
    let value = r#"
	{
		"hello": "world",
		"hehe": "hoho"
	}
	"#;

    let mut deserializer = serde_json::Deserializer::from_str(value);
    schema.builder().deserialize(&mut deserializer).unwrap();
}

#[test]
fn incomplete_with_optional2() {
    let nested_string = MessyJson::String(MessyJsonScalar::new(false));
    let nested_string_opt = MessyJson::String(MessyJsonScalar::new(true));
    let schema: MessyJson = MessyJson::Obj(MessyJsonObject::new(
        vec![
            ("hello".to_string(), nested_string.clone()),
            ("whoami".to_string(), nested_string),
            ("hehe".to_string(), nested_string_opt),
        ]
        .into_iter()
        .collect(),
        false,
    ));
    let value = r#"
	{
		"hello": "world",
		"whoami": "hoho"
	}
	"#;

    let mut deserializer = serde_json::Deserializer::from_str(value);
    schema.builder().deserialize(&mut deserializer).unwrap();
}

#[test]
fn all_optional() {
    let nested_string_opt = MessyJson::String(MessyJsonScalar::new(true));
    let schema: MessyJson = MessyJson::Obj(MessyJsonObject::new(
        vec![
            ("hello".to_string(), nested_string_opt.clone()),
            ("whoami".to_string(), nested_string_opt.clone()),
            ("hehe".to_string(), nested_string_opt),
        ]
        .into_iter()
        .collect(),
        false,
    ));
    let value = r#"
	{
	}
	"#;

    let mut deserializer = serde_json::Deserializer::from_str(value);
    schema.builder().deserialize(&mut deserializer).unwrap();
}

#[test]
fn nested_missing() {
    let nested_string = MessyJson::String(MessyJsonScalar::new(false));
    let schema: MessyJson = MessyJson::Obj(MessyJsonObject::new(
        vec![
            (
                "hello".to_string(),
                MessyJson::Obj(MessyJsonObject::new(
                    vec![("world".to_string(), nested_string.clone())]
                        .into_iter()
                        .collect(),
                    false,
                )),
            ),
            ("whoami".to_string(), nested_string.clone()),
            ("hehe".to_string(), nested_string),
        ]
        .into_iter()
        .collect(),
        false,
    ));
    let value = r#"
	{
		"hello": {},
		"whoami": "hello",
		"hehe": "hoho"
	}
	"#;

    let mut deserializer = serde_json::Deserializer::from_str(value);
    let parsed = schema.builder().deserialize(&mut deserializer).unwrap_err();
    println!("{:#?}", parsed);
}

#[test]
fn nested_unknown() {
    let nested_string = MessyJson::String(MessyJsonScalar::new(false));
    let schema: MessyJson = MessyJson::Obj(MessyJsonObject::new(
        vec![
            (
                "hello".to_string(),
                MessyJson::Obj(MessyJsonObject::new(
                    vec![("world".to_string(), nested_string.clone())]
                        .into_iter()
                        .collect(),
                    false,
                )),
            ),
            ("whoami".to_string(), nested_string.clone()),
            ("hehe".to_string(), nested_string),
        ]
        .into_iter()
        .collect(),
        false,
    ));
    let value = r#"
	{
		"hello": {
			"world": "HAAAAAAAAAAAAAAAAAAAAAA",
			"wtfiamdoingwithmylife": "that's a very good question dude" 
		},
		"whoami": "hello",
		"hehe": "hoho"
	}
	"#;

    let mut deserializer = serde_json::Deserializer::from_str(value);
    let parsed = schema.builder().deserialize(&mut deserializer).unwrap_err();
    println!("{:#?}", parsed);
}

#[test]
fn nested_optional() {
    let nested_string_opt = MessyJson::String(MessyJsonScalar::new(true));
    let nested_string = MessyJson::String(MessyJsonScalar::new(false));
    let schema: MessyJson = MessyJson::Obj(MessyJsonObject::new(
        vec![
            (
                "hello".to_string(),
                MessyJson::Obj(MessyJsonObject::new(
                    vec![("world".to_string(), nested_string_opt)]
                        .into_iter()
                        .collect(),
                    false,
                )),
            ),
            ("whoami".to_string(), nested_string.clone()),
            ("hehe".to_string(), nested_string),
        ]
        .into_iter()
        .collect(),
        false,
    ));
    let value = r#"
	{
		"hello": {},
		"whoami": "hello",
		"hehe": "hoho"
	}
	"#;

    let mut deserializer = serde_json::Deserializer::from_str(value);
    schema.builder().deserialize(&mut deserializer).unwrap();
}

#[test]
fn nested_optional_parent_optional() {
    let nested_string_opt = MessyJson::String(MessyJsonScalar::new(true));
    let nested_string = MessyJson::String(MessyJsonScalar::new(false));
    let schema: MessyJson = MessyJson::Obj(MessyJsonObject::new(
        vec![
            (
                "hello".to_string(),
                MessyJson::Obj(MessyJsonObject::new(
                    vec![("world".to_string(), nested_string_opt)]
                        .into_iter()
                        .collect(),
                    true, // Optional parent
                )),
            ),
            ("whoami".to_string(), nested_string.clone()),
            ("hehe".to_string(), nested_string),
        ]
        .into_iter()
        .collect(),
        false,
    ));
    let value = r#"
	{
		"whoami": "hello",
		"hehe": "hoho"
	}
	"#;

    let mut deserializer = serde_json::Deserializer::from_str(value);
    schema.builder().deserialize(&mut deserializer).unwrap();
}
