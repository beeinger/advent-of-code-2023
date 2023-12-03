use advent_of_code_2023::day_2::{part_1, part_2};
use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};

fn bench(c: &mut Criterion) {
    let mut group = c.benchmark_group("day_2");
    let input = part_1::get_input();

    group.bench_with_input(BenchmarkId::new("part 1", "puzzle"), &input, |b, _| {
        b.iter(|| part_1::sum_possible_ids(&input));
    });

    group.bench_with_input(BenchmarkId::new("part 2", "puzzle"), &input, |b, _| {
        b.iter(|| part_2::sum_powers_of_min_sets(&input));
    });

    group.finish();
}

criterion_group!(benches, bench);
criterion_main!(benches);
