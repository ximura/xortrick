use criterion::{BenchmarkId, Criterion, black_box, criterion_group, criterion_main};
use xortrick::{find_missing, find_missing_iter, find_missing_opt};

fn bench_find_missing(c: &mut Criterion) {
    let mut group = c.benchmark_group("find_missing");
    let n = 100_000;
    let input: Vec<i32> = (1..=n).filter(|&x| x != 789).collect();

    for i in [20u64, 21u64].iter() {
        group.bench_with_input(BenchmarkId::new("simple solution", i), i, |b, _| {
            b.iter(|| find_missing(black_box(&input), black_box(n)))
        });
        group.bench_with_input(BenchmarkId::new("iter solution", i), i, |b, _| {
            b.iter(|| find_missing_iter(black_box(&input), black_box(n)))
        });
        group.bench_with_input(BenchmarkId::new("optimized solution", i), i, |b, _| {
            b.iter(|| find_missing_opt(black_box(&input), black_box(n)))
        });
    }
    group.finish();
}

criterion_group!(benches, bench_find_missing);
criterion_main!(benches);
