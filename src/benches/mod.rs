use criterion::{black_box, criterion_group, criterion_main, Criterion};
use std::borrow::Cow;

extern crate messy_json;
use messy_json::*;

mod vs_serde;

criterion_group!(benches, vs_serde::criterion_benchmark);
criterion_main!(benches);
