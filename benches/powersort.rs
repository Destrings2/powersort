use criterion::*;
use powersort::{alternatives, powersort_alternatives};
use powersort::sequences::{generate_runs_with_average_length, generate_random_sequence, generate_timsort_drag};
use powersort::powersort::power_sort;

fn benchmark_powersort_1(c: &mut Criterion) {
    let mut is_less = |a: &i32, b: &i32| a < b;
    let mut group = c.benchmark_group("powersort_run");
    for size in [1000usize, 10000, 100000, 1000000].into_iter() {
       for average_length_percentage in [0.01, 0.025, 0.05].into_iter() {
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

            group.bench_function(BenchmarkId::new(format!("powersort_buffer_{}_run", average_length_percentage), size), 
            |b| {
                b.iter_batched(|| {
                    sequence.clone()
                }, |mut v: Vec<i32>| {
                    powersort_alternatives::power_sort_buffer(&mut v, &mut is_less);
                }
                , BatchSize::SmallInput);
            });

            group.bench_function(BenchmarkId::new(format!("powersort_left_{}_run", average_length_percentage), size), 
            |b| {
                b.iter_batched(|| {
                    sequence.clone()
                }, |mut v: Vec<i32>| {
                    powersort_alternatives::power_sort_left(&mut v, &mut is_less);
                }
                , BatchSize::SmallInput);
            });

            group.bench_function(BenchmarkId::new(format!("powersort_no_insert_{}_run", average_length_percentage), size), 
            |b| {
                b.iter_batched(|| {
                    sequence.clone()
                }, |mut v: Vec<i32>| {
                    powersort_alternatives::power_sort_no_insertion(&mut v, &mut is_less);
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
    for size in [1000usize, 10000, 100000].into_iter() {
        let sequence = generate_random_sequence(size);

        group.bench_function(BenchmarkId::new("powersort_run", size), |b| {
            b.iter_batched(|| {
                sequence.clone()
            }, |mut v: Vec<i32>| {
                power_sort(&mut v, &mut is_less);
            }
            , BatchSize::SmallInput);
        });

        group.bench_function(BenchmarkId::new("powersort_buffer_run", size), |b| {
            b.iter_batched(|| {
                sequence.clone()
            }, |mut v: Vec<i32>| {
                powersort_alternatives::power_sort_buffer(&mut v, &mut is_less);
            }
            , BatchSize::SmallInput);
        });

        group.bench_function(BenchmarkId::new("powersort_left_run", size), |b| {
            b.iter_batched(|| {
                sequence.clone()
            }, |mut v: Vec<i32>| {
                powersort_alternatives::power_sort_left(&mut v, &mut is_less);
            }
            , BatchSize::SmallInput);
        });

        group.bench_function(BenchmarkId::new("powersort_no_insert_run", size), |b| {
            b.iter_batched(|| {
                sequence.clone()
            }, |mut v: Vec<i32>| {
                powersort_alternatives::power_sort_no_insertion(&mut v, &mut is_less);
            }
            , BatchSize::SmallInput);
        });
    }
    group.finish();
}

criterion_group!(powersort_group, benchmark_powersort_1, benchmark_powersort_2);
criterion_main!(powersort_group);