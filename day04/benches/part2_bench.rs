use std::{hint::black_box, time::Duration};

use criterion::{Criterion, criterion_group, criterion_main};
use day04::part2;
use utils::get_input;

fn criterion_benchmark(c: &mut Criterion) {
  let input = get_input(env!("CARGO_MANIFEST_DIR")).unwrap();
  let mut group = c.benchmark_group("day 04");

  group.bench_function("part 2", |b| b.iter(|| part2::run(black_box(&input))));

  group.finish();
}

criterion_group!(
  name = benches;
  config = Criterion::default()
    .sample_size(100)
    .measurement_time(Duration::from_secs(32))
    .warm_up_time(Duration::from_secs(1));
  targets = criterion_benchmark
);
criterion_main!(benches);
