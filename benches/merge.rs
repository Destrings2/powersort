use criterion::*;
use powersort::powersort::merge;
use powersort::sequences::*;


pub fn benchmark_merge_1(c: &mut Criterion) {
    let mut is_less = |a: &i32, b: &i32| a < b;

    let mut group = c.benchmark_group("Merge sizes");
    for size in [10usize, 100, 1000, 10000, 100000, 200000, 300000].into_iter() {
        group.bench_function(BenchmarkId::new("Merge", size), 
        |b| {
            b.iter_batched(
                || {
                    generate_runs_with_lengths(size, &[size/2, size/2])
                },
                |mut v: Vec<i32>| {
                    merge(&mut v, size/2+1, &mut is_less);
                }
            , BatchSize::SmallInput);
        });
    }
    
}

criterion_group!(merge_group, benchmark_merge_1);
criterion_main!(merge_group);