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


