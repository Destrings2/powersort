use criterion::*;
use powersort::powersort::merge;
use powersort::sort::merge as default_merge;
use powersort::sequences::*;


pub fn benchmark_merge_1(c: &mut Criterion) {
    let mut is_less = |a: &i32, b: &i32| a < b;

    let mut group = c.benchmark_group("merge_benchmark");
    for size in [10usize, 100, 1000, 10000, 100000, 200000, 300000].into_iter() {
        let mut buf = Vec::with_capacity(size/2);
        let run = generate_runs_with_lengths(size, &[size/2, size/2]);

        group.bench_function(BenchmarkId::new("merge", size), 
        |b| {
            b.iter_batched(
                || {
                    run.clone()
                },
                |mut v: Vec<i32>| {
                    merge(&mut v, size/2+1, &mut is_less);
                }
            , BatchSize::SmallInput);
        });

        group.bench_function(BenchmarkId::new("safe_merge", size), |b|{
            b.iter_batched(
                || {
                    run.clone()
                },
                |mut v: Vec<i32>| {
                    unsafe {
                        default_merge(&mut v, size/2+1, buf.as_mut_ptr(), &mut is_less);
                    }
                }
                , BatchSize::SmallInput)
        });
    }
    group.finish();
}

criterion_group!(merge_group, benchmark_merge_1);
criterion_main!(merge_group);