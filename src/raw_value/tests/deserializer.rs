use super::*;
use serde::Deserialize;

const VAL: &str = r#"
{
	"person": {
		"age": 12,
		"isachild": true,
		"gender": "O",
		"name": "AAAAAAAAAAAAAA"
	},
	"employment": "not_working",
	"friends": ["Paul", "Paula", "Paulo", "Pau", "Potdeterrecuitemalcuite"]
}
"#;

#[derive(Clone, Debug, Deserialize)]
struct DummyPerson {
    age: u16,
    isachild: bool,
    gender: char,
    name: Option<String>,
    surname: Option<String>,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
enum DummyJob {
    Unemployed,
    NotWorking,
}

#[derive(Clone, Debug, Deserialize)]
struct DummyStruct {
    person: DummyPerson,
    employment: DummyJob,
    friends: Vec<String>,
}

#[test]
fn complexe() {
    let val: MessyJsonValueRaw<'_> = serde_json::from_str(VAL).unwrap();
    let res: DummyStruct = DummyStruct::deserialize(val).unwrap();

    assert_eq!(res.person.age, 12);
    assert_eq!(res.person.isachild, true);
    assert_eq!(res.person.gender, 'O');
    assert_eq!(
        matches!(res.person.name, Some(x) if x == "AAAAAAAAAAAAAA"),
        true
    );
    assert_eq!(res.person.surname.is_none(), true);
    assert_eq!(
        res.friends,
        vec!["Paul", "Paula", "Paulo", "Pau", "Potdeterrecuitemalcuite"]
            .into_iter()
            .map(str::to_string)
            .collect::<Vec<String>>()
    );
}
