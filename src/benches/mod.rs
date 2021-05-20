use criterion::{criterion_group, criterion_main, Criterion};
use std::borrow::Cow;
use std::time::Duration;

extern crate messy_json;
use messy_json::*;
mod vs_serde_dummy_obj;
mod vs_serde_obj;
mod vs_serde_optional_obj;
mod vs_serde_str;

pub fn apply_criterion_group_settings<T: criterion::measurement::Measurement>(
    group: &mut criterion::BenchmarkGroup<T>,
) {
    group.sample_size(1000);
    group.warm_up_time(Duration::from_secs(5));
    group.measurement_time(Duration::from_secs(20));
}

criterion_group!(
    benches,
    vs_serde_obj::criterion_benchmark,
    vs_serde_str::criterion_benchmark
);
criterion_main!(benches);
