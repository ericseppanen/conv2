use conv2_benchmarks::{asm_f32_i32, conv2_f32_u32, num_f32_u32};
use criterion::{black_box, criterion_group, criterion_main, BatchSize, Criterion};

fn make_input() -> Vec<f32> {
    let mut input = Vec::<f32>::new();
    input.resize_with(100000, || rand::random());
    input
}

pub fn convert_benchmark(c: &mut Criterion) {
    c.bench_function("num_f32_u32", |b| {
        b.iter_batched(make_input, num_f32_u32, BatchSize::LargeInput)
    });
    c.bench_function("conv2_f32_u32", |b| {
        b.iter_batched(make_input, conv2_f32_u32, BatchSize::LargeInput)
    });
    c.bench_function("asm_f32_i32", |b| {
        b.iter_batched(make_input, asm_f32_i32, BatchSize::LargeInput)
    });
}

criterion_group!(benches, convert_benchmark);
criterion_main!(benches);
