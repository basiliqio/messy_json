use super::vs_serde_dummy_obj;
use super::vs_serde_optional_obj;
use super::*;

pub fn criterion_benchmark(c: &mut Criterion) {
    vs_serde_dummy_obj::criterion_benchmark(c);
    vs_serde_optional_obj::criterion_benchmark(c);
}
