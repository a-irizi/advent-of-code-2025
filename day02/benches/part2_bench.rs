use std::{hint::black_box, time::Duration};

use criterion::{Criterion, criterion_group, criterion_main};
use day02::{part2, part2_leading, part2_parallel};
use utils::get_input;

fn criterion_benchmark(c: &mut Criterion) {
  let input = get_input(env!("CARGO_MANIFEST_DIR")).unwrap();
  let mut group = c.benchmark_group("day 02 part 2");

  group.bench_function("trailing digit", |b| b.iter(|| part2::run(black_box(&input))));
  group.bench_function("trailing digit parallel", |b| {
    b.iter(|| part2_parallel::run(black_box(&input)));
  });
  group.bench_function("leading digit", |b| b.iter(|| part2_leading::run(black_box(&input))));

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
