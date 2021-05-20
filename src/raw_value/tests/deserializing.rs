use super::*;

#[test]
fn simple_deserialize() {
    const VAL: &str = r#"
	{
		"string": "world",
		"bool": true,
		"number": 15,
		"inumber": -15,
		"null": null
	}
	"#;

    let val: MessyJsonValueRaw<'_> = serde_json::from_str(VAL).unwrap();

    match val {
        MessyJsonValueRaw::Obj(obj) => {
            assert_eq!(
                matches!(obj.get("string").unwrap(), MessyJsonValueRaw::String(x) if x == "world"),
                true
            );
            assert_eq!(
                matches!(obj.get("bool").unwrap(), MessyJsonValueRaw::Bool(x) if x == &true),
                true
            );
            assert_eq!(
                matches!(obj.get("number").unwrap(), MessyJsonValueRaw::Number(x) if x == &15),
                true
            );
            assert_eq!(
                matches!(obj.get("inumber").unwrap(), MessyJsonValueRaw::Number(x) if *x as i64 == -15),
                true
            );
            assert_eq!(
                matches!(obj.get("null").unwrap(), MessyJsonValueRaw::Null),
                true
            );
        }
        _ => panic!("should've been an object"),
    }
}

#[test]
fn nested_array_deserialize() {
    const VAL: &str = r#"
	{
		"array": [
			"A string",
			12,
			null,
			true
		]
	}
	"#;

    let val: MessyJsonValueRaw<'_> = serde_json::from_str(VAL).unwrap();

    match val {
        MessyJsonValueRaw::Obj(obj) => match obj.get("array").unwrap() {
            MessyJsonValueRaw::Array(arr) => {
                assert_eq!(
                    matches!(&arr[0], MessyJsonValueRaw::String(x) if x == "A string"),
                    true
                );
                assert_eq!(
                    matches!(&arr[1], MessyJsonValueRaw::Number(x) if *x == 12),
                    true
                );
                assert_eq!(matches!(&arr[2], MessyJsonValueRaw::Null), true);
                assert_eq!(matches!(&arr[3], MessyJsonValueRaw::Bool(x) if *x), true);
            }
            _ => panic!("Should've been an array"),
        },
        _ => panic!("should've been an object"),
    }
}

#[test]
fn nested_obj_deserialize() {
    const VAL: &str = r#"
	{
		"object": {
			"hello": "world",
			"hello2": "world2"
		}
	}
	"#;

    let val: MessyJsonValueRaw<'_> = serde_json::from_str(VAL).unwrap();

    match val {
        MessyJsonValueRaw::Obj(obj) => match obj.get("object").unwrap() {
            MessyJsonValueRaw::Obj(obj) => {
                assert_eq!(
                    matches!(obj.get("hello").unwrap(), MessyJsonValueRaw::String(x) if x == "world"),
                    true
                );
                assert_eq!(
                    matches!(obj.get("hello2").unwrap(), MessyJsonValueRaw::String(x) if x == "world2"),
                    true
                );
            }
            _ => panic!("Should've been an object"),
        },
        _ => panic!("should've been an object"),
    }
}
