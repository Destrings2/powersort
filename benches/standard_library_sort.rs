use criterion::*;
use powersort::sequences::{generate_runs_with_average_length, generate_random_sequence, generate_timsort_drag};
use powersort::sort::merge_sort;

fn benchmark_standardsort_1(c: &mut Criterion) {
    let mut is_less = |a: &i32, b: &i32| a < b;
    let mut group = c.benchmark_group("standard_powersort_runs");
    for size in [1000usize, 10000, 100000, 1000000, 100000000].into_iter() {
       for average_length_percentage in [0.01, 0.025, 0.05, 0.1, 0.2, 0.5].into_iter() {
        let average_length = (size as f64 * average_length_percentage).ceil() as usize;
        let sequence = generate_runs_with_average_length(size, size*average_length);

        group.bench_function(BenchmarkId::new(format!("standard_powersort_{}_run", average_length_percentage), size), |b| {
            b.iter_batched(|| {
                sequence.clone()
            }, |mut v: Vec<i32>| {
                merge_sort(&mut v, &mut is_less);
            }
            , BatchSize::SmallInput);
        });

        group.bench_function(BenchmarkId::new(format!("standard_timsort_{}_run", average_length_percentage), size), |b| {
            b.iter_batched(|| {
                sequence.clone()
            }, |mut v: Vec<i32>| {
                v.sort();
            }
            , BatchSize::SmallInput);
        });
       }
    }
    group.finish();
}

fn benchmark_standardsort_2(c: &mut Criterion) {
    let mut is_less = |a: &i32, b: &i32| a < b;
    let mut group = c.benchmark_group("standard_powersort_random");
    for size in [1000usize, 10000, 100000, 1000000, 100000000].into_iter() {
        let sequence = generate_random_sequence(size);

        group.bench_function(BenchmarkId::new("standard_powersort_run", size), |b| {
            b.iter_batched(|| {
                sequence.clone()
            }, |mut v: Vec<i32>| {
                merge_sort(&mut v, &mut is_less);
            }
            , BatchSize::SmallInput);
        });

        group.bench_function(BenchmarkId::new("standard_timsort_run", size), |b| {
            b.iter_batched(|| {
                sequence.clone()
            }, |mut v: Vec<i32>| {
                v.sort();
            }
            , BatchSize::SmallInput);
        });
    }
    group.finish();
}

fn benchmark_standardsort_3(c: &mut Criterion) {
    let mut is_less = |a: &i32, b: &i32| a < b;
    let mut group = c.benchmark_group("standard_powersort_drag");
    for size in [1000usize, 10000, 100000, 1000000, 100000000].into_iter() {
        let sequence = generate_timsort_drag(size, 10);

        group.bench_function(BenchmarkId::new("standard_powersort_run", size), |b| {
            b.iter_batched(|| {
                sequence.clone()
            }, |mut v: Vec<i32>| {
                merge_sort(&mut v, &mut is_less);
            }
            , BatchSize::SmallInput);
        });

        group.bench_function(BenchmarkId::new("standard_timsort_run", size), |b| {
            b.iter_batched(|| {
                sequence.clone()
            }, |mut v: Vec<i32>| {
                v.sort();
            }
            , BatchSize::SmallInput);
        });
    }
    group.finish();
}

criterion_group!(standard_sort_group, benchmark_standardsort_1, benchmark_standardsort_2, benchmark_standardsort_3);
criterion_main!(standard_sort_group);