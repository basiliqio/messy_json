# Messy Json

**Rust JSON Parser for dynamically structured documents**

<a href="https://gitlab.com/basiliq/messy_json/-/pipelines" alt="Gitlab pipeline status">
  <img src="https://img.shields.io/gitlab/pipeline/basiliq/messy_json/master">
</a>
<a href="https://codecov.io/gl/basiliq/messy_json" alt="Codecov">
  <img src="https://img.shields.io/codecov/c/gitlab/basiliq/messy_json?token=tnPbnqeTHj">
</a>
<a href="https://crates.io/crates/messy_json" alt="Crates.io version">
  <img src="https://img.shields.io/crates/v/messy_json">
</a>
<a href="https://crates.io/crates/messy_json" alt="Crates.io license">
  <img src="https://img.shields.io/crates/l/messy_json?label=license">
</a>
<a href="https://docs.rs/messy_json" alt="Docs.rs">
  <img src="https://docs.rs/messy_json/badge.svg">
</a>

- [Messy Json](#messy-json)
	- [Introduction](#introduction)
	- [Example](#example)
	- [Performance](#performance)
		- [Dummy object](#dummy-object)
		- [Partial object](#partial-object)
		- [Simple object](#simple-object)

## Introduction

The rust ecosystem allows for **very** good compile-time implementation of JSON deserializer to rust structure, however,
things get a bit more sparse when it come to run-time deserialization of dynamically structured objects.
This crate approaches this problems in a simple manner, resembling [`serde_json`'s `Value`](https://docs.serde.rs/serde_json/value/enum.Value.html).

## Example

```rust
    let nested_string = MessyJson::String(MessyJsonScalar::new(false));
    let schema: MessyJson = MessyJson::Obj(Box::new(MessyJsonObject::new(
        vec![("hello".to_string(), nested_string)]
            .into_iter()
            .collect(),
        false,
    )));
    let value = r#"
	{
		"hello": "world"
	}
	"#;

	let mut deserializer = serde_json::Deserializer::from_str(value);
	let parsed: MessyJsonValueContainer = schema.builder().deserialize(&mut deserializer).unwrap();
	
	println!("{:#?}", parsed)
```

## Performance

This crate is more effecient than [`serde_json`'s `Value`](https://docs.serde.rs/serde_json/value/enum.Value.html) when all the fields are required. The performance par with [`serde_json`'s `Value`](https://docs.serde.rs/serde_json/value/enum.Value.html) when some fields are optional.

However this crate is far behind deserializing using the `proc-macro` from serde (which is not dynamically structured at all).

This gap could be filled using a custom arena-based allocator, like [Bumpalo](https://crates.io/crates/bumpalo) when the `Allocator` trait is merged into `stable`. 

This crate implements benchmarks.
The following graphs were run on a machine with the following specs:

- CPU		: Intel i9-9900K @ 5Ghz
- RAM		: 32 Gb RAM @ 2133 Mhz
- Kernel	: `5.10.10-arch1-1`
- Rust		: `rustc 1.49.0 (e1884a8e3 2020-12-29)`

In the following benchmarks, the `messy_json` crate is compared with deserializer from the [`serde_json`'s `Value`](https://docs.serde.rs/serde_json/value/enum.Value.html) and macro-generated deserializer using `serde`'s `derive`.

### Dummy object

The following benchmark consists of deserializing the JSON Document

```json
{
	"hello":
	{
		"hola": "world"
	}
}
```

the accepted schema should looks like the following:

```rust
struct DummyObjNested<'a> {
    hola: Cow<'a, str>,
}

struct DummyObj<'a> {
    hello: DummyObjNested<'a>,
}
```

The results show that `messy_json` is slower than macro-generated deserializer but faster than using
[`serde_json`'s `Value`](https://docs.serde.rs/serde_json/value/enum.Value.html).

![Dummy structure violin](./benches/dummy_violin.svg)

### Partial object

The following benchmark consists of deserializing the JSON Document

```json
{
	"hello":
	{
		"hola": "world"
	}
}
```

the accepted schema should looks like the following:

```rust
#[derive(Serialize, Deserialize)]
struct PartialObjNested<'a> {
    hola: Cow<'a, str>,
}

#[derive(Serialize, Deserialize)]
struct PartialObj<'a> {
    hello: PartialObjNested<'a>,
    coucou: Option<Cow<'a, str>>,
    coucou1: Option<Cow<'a, str>>,
    coucou2: Option<Cow<'a, str>>,
}
```

The results show that `messy_json` is slower than macro-generated deserializer and on par with [`serde_json`'s `Value`](https://docs.serde.rs/serde_json/value/enum.Value.html). When using optional values, this crate has to check it has met all of the mandatory values for each object, hence the performance regression. In the future, when the `alloc_api` of the Rust language is merged into `stable`, optimizations could be put in place reducing the time necessary to check for missing fields.

![Partial structure violin](./benches/partial_violin.svg)

### Simple object

The following benchmark consists of deserializing the JSON Document

```json
{
	"hello": "world"
}
```

the accepted schema should looks like the following:

```rust
#[derive(Serialize, Deserialize)]
struct SimpleObj<'a> {
    hello: Cow<'a, str>,
}
```

The results show that `messy_json` is slower than macro-generated deserializer but is still faster than [`serde_json`'s `Value`](https://docs.serde.rs/serde_json/value/enum.Value.html). 

![Partial structure violin](./benches/simple_violin.svg)
