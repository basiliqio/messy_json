use criterion::{criterion_group, criterion_main, Criterion};
use std::borrow::Cow;

extern crate messy_json;
use messy_json::*;

mod vs_serde_dummy_obj;
mod vs_serde_obj;
mod vs_serde_optional_obj;
mod vs_serde_str;

criterion_group!(benches_obj, vs_serde_obj::criterion_benchmark);
criterion_group!(benches_str, vs_serde_str::criterion_benchmark);
criterion_main!(benches_obj, benches_str);
