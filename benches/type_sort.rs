use criterion::*;
use powersort::sequences::{generate_runs_with_average_length, generate_random_sequence};

fn benchmark_type_sort_1(c: &mut Criterion) {
    let mut group = c.benchmark_group("default_sort_runs");
    for size in [1000usize, 10000, 100000, 1000000].into_iter() {
       for average_length_percentage in [0.01, 0.025, 0.05].into_iter() {
        let average_length = (size as f64 * average_length_percentage).ceil() as usize;
        let sequence = generate_runs_with_average_length(size, size*average_length);

        group.bench_function(BenchmarkId::new(format!("default_sort_{}_run", average_length_percentage), size), |b| {
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

fn benchmark_type_sort_2(c: &mut Criterion) {
    let mut group = c.benchmark_group("default_sort_random_runs");
    for size in [1000usize, 10000, 100000, 1000000].into_iter() {
        let sequence = generate_random_sequence(size);

        group.bench_function(BenchmarkId::new("default_sort_random_run", size), |b| {
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

criterion_group!(type_sort_group, benchmark_type_sort_1, benchmark_type_sort_2);
criterion_main!(type_sort_group);