use criterion::{criterion_group, criterion_main, Criterion};
{% if crate_type == "lib" %}
pub fn add_benchmark(c: &mut Criterion) {
    let mut rvg = {{crate_name}}::test_utils::Rvg::deterministic();
    let int_val_1 = rvg.sample(&(0..100i32));
    let int_val_2 = rvg.sample(&(0..100i32));

    c.bench_function("add", |b| {
        b.iter(|| {
            {{crate_name}}::add(int_val_1, int_val_2);
        })
    });
}
{% else %}
pub fn add_benchmark(c: &mut Criterion) {
    let mut rvg = {{crate_name}}::test_utils::Rvg::deterministic();
    let int_val_1 = rvg.sample(&(0..100i32));
    let int_val_2 = rvg.sample(&(0..100i32));

    c.bench_function("add", |b| {
        b.iter(|| {
            let _ = int_val_1 + int_val_2;
        })
    });
}
{% endif %}
criterion_group!(benches, add_benchmark);
criterion_main!(benches);
