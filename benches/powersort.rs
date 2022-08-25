use criterion::*;
use powersort::sequences::{generate_runs_with_average_length, generate_random_sequence, generate_timsort_drag};
use powersort::powersort::power_sort;

fn benchmark_powersort_1(c: &mut Criterion) {
    let mut is_less = |a: &i32, b: &i32| a < b;
    let mut group = c.benchmark_group("powersort_run");
    for size in [1000usize, 10000, 100000, 1000000, 100000000].into_iter() {
       for average_length_percentage in [0.01, 0.025, 0.05, 0.1, 0.2, 0.5].into_iter() {
            let average_length = (size as f64 * average_length_percentage).ceil() as usize;
            let sequence = generate_runs_with_average_length(size, size*average_length);

            group.bench_function(BenchmarkId::new(format!("powersort_{}_run", average_length_percentage), size), 
            |b| {
                b.iter_batched(|| {
                    sequence.clone()
                }, |mut v: Vec<i32>| {
                    power_sort(&mut v, &mut is_less);
                }
                , BatchSize::SmallInput);
            });
        }
    }
    group.finish();
}


fn benchmark_powersort_2(c: &mut Criterion) {
    let mut is_less = |a: &i32, b: &i32| a < b;
    let mut group = c.benchmark_group("powersort_random");
    for size in [1000usize, 10000, 100000, 1000000, 100000000].into_iter() {
        let sequence = generate_random_sequence(size);

        group.bench_function(BenchmarkId::new("powersort_run", size), |b| {
            b.iter_batched(|| {
                sequence.clone()
            }, |mut v: Vec<i32>| {
                power_sort(&mut v, &mut is_less);
            }
            , BatchSize::SmallInput);
        });
    }
    group.finish();
}

fn benchmark_powersort_3(c: &mut Criterion) {
    let mut is_less = |a: &i32, b: &i32| a < b;
    let mut group = c.benchmark_group("powersort_drag");
    for size in [1000usize, 10000, 100000, 1000000, 100000000].into_iter() {
        let sequence = generate_timsort_drag(size, 10);

        group.bench_function(BenchmarkId::new("powersort_run", size), |b| {
            b.iter_batched(|| {
                sequence.clone()
            }, |mut v: Vec<i32>| {
                power_sort(&mut v, &mut is_less);
            }
            , BatchSize::SmallInput);
        });
    }
    group.finish();
}

criterion_group!(powersort_group, benchmark_powersort_1, benchmark_powersort_2, benchmark_powersort_3);
criterion_main!(powersort_group);