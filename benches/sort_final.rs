use powersort::sequences::{generate_runs_with_average_length, generate_random_sequence};
use powersort::powersort_final::power_sort as merge_sort;
use criterion::*;

fn benchmark_finalsort_1(c: &mut Criterion) {
    let mut is_less = |a: &i32, b: &i32| a < b;
    let mut group = c.benchmark_group("final_powersort_runs");
    for size in [1000usize, 10000, 100000, 1000000, 10000000, 100000000].into_iter() {
       for average_length_percentage in [0.01, 0.025, 0.05].into_iter() {
        let average_length = (size as f64 * average_length_percentage).ceil() as usize;
        let sequence = generate_runs_with_average_length(size, size*average_length);

        group.bench_function(BenchmarkId::new(format!("final_powersort_{}_run", average_length_percentage), size), |b| {
            b.iter_batched(|| {
                sequence.clone()
            }, |mut v: Vec<i32>| {
                merge_sort(&mut v, &mut is_less);
            }
            , BatchSize::SmallInput);
        });
       }
    }
    group.finish();
}

fn benchmark_finalsort_2(c: &mut Criterion) {
    let mut is_less = |a: &i32, b: &i32| a < b;
    let mut group = c.benchmark_group("final_powersort_random");
    for size in [1000usize, 10000, 100000, 10000000].into_iter() {
        let sequence = generate_random_sequence(size);

        group.bench_function(BenchmarkId::new("final_powersort_run", size), |b| {
            b.iter_batched(|| {
                sequence.clone()
            }, |mut v: Vec<i32>| {
                merge_sort(&mut v, &mut is_less);
            }
            , BatchSize::SmallInput);
        });
    }
    group.finish();
}


criterion_group!(final_sort_group, benchmark_finalsort_1, benchmark_finalsort_2);
criterion_main!(final_sort_group);