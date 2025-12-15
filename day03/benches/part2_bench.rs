use std::{hint::black_box, time::Duration};

use criterion::{Criterion, criterion_group, criterion_main};
use day03::{part2, part2_const_bank_size, part2_const_bank_size_par};
use utils::get_input;

fn criterion_benchmark(c: &mut Criterion) {
  let input = get_input(env!("CARGO_MANIFEST_DIR")).unwrap();
  let mut group = c.benchmark_group("day 03 part 2");

  group.bench_function("generic bank size", |b| b.iter(|| part2::run(black_box(&input))));
  group.bench_function("const bank size", |b| {
    b.iter(|| part2_const_bank_size::run::<100>(black_box(&input)));
  });
  group.bench_function("const bank size parallel", |b| {
    b.iter(|| part2_const_bank_size_par::run::<100>(black_box(&input)));
  });

  group.finish();
}

criterion_group!(
  name = benches;
  config = Criterion::default()
    .sample_size(100)
    .measurement_time(Duration::from_secs(30))
    .warm_up_time(Duration::from_secs(1));
  targets = criterion_benchmark
);
criterion_main!(benches);
