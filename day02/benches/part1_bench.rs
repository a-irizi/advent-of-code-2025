use std::{hint::black_box, time::Duration};

use criterion::{Criterion, criterion_group, criterion_main};
use day02::{part1, part1_leading};
use utils::get_input;

fn criterion_benchmark(c: &mut Criterion) {
  let input = get_input(env!("CARGO_MANIFEST_DIR")).unwrap();
  let mut group = c.benchmark_group("day 02 part 1");

  group.bench_function("trailing digit", |b| b.iter(|| part1::run(black_box(&input))));
  group.bench_function("leading digit", |b| b.iter(|| part1_leading::run(black_box(&input))));

  group.finish();
}

criterion_group!(
  name = benches;
  config = Criterion::default()
    .sample_size(100)
    .measurement_time(Duration::from_secs(25))
    .warm_up_time(Duration::from_secs(1));
  targets = criterion_benchmark
);
criterion_main!(benches);
