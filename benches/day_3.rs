use advent_of_code_2023::day_3::{part_1, part_2};
use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};

fn bench(c: &mut Criterion) {
    let mut group = c.benchmark_group("day_3");
    let input = part_1::get_input();

    group.bench_with_input(BenchmarkId::new("part 1", "puzzle"), &input, |b, _| {
        b.iter(|| part_1::sum_part_nums(&input));
    });

    group.bench_with_input(BenchmarkId::new("part 2", "puzzle"), &input, |b, _| {
        b.iter(|| part_2::sum_gear_ratios(&input));
    });

    group.finish();
}

criterion_group!(benches, bench);
criterion_main!(benches);
