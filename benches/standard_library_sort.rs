use std::mem::size_of;

use criterion::*;
use powersort::sequences::{generate_runs_with_average_length, generate_random_sequence, generate_timsort_drag};
use powersort::sort::*;

pub fn tim_sort<T, F>(v: &mut [T], mut is_less: F) -> usize
where
    F: FnMut(&T, &T) -> bool,
{
    let mut merge_cost = 0;
    const MAX_INSERTION: usize = 20;
    const MIN_RUN: usize = 10;

    if size_of::<T>() == 0 {
        return 0;
    }

    let len = v.len();

    if len <= MAX_INSERTION {
        if len >= 2 {
            for i in (0..len - 1).rev() {
                insert_head(&mut v[i..], &mut is_less);
            }
        }
        return 0;
    }

    let mut buf = Vec::with_capacity(len / 2);

    let mut runs = vec![];
    let mut end = len;
    while end > 0 {
        let mut start = end - 1;
        if start > 0 {
            start -= 1;
            unsafe {
                if is_less(v.get_unchecked(start + 1), v.get_unchecked(start)) {
                    while start > 0 &&
                    is_less(v.get_unchecked(start), v.get_unchecked(start - 1)) {
                        start -= 1;
                    }
                    v[start..end].reverse();
                } else {
                    while start > 0 && 
                    !is_less(v.get_unchecked(start), v.get_unchecked(start - 1))
                    {
                        start -= 1;
                    }
                }
            }
        }

        while start > 0 && end - start < MIN_RUN {
            start -= 1;
            insert_head(&mut v[start..end], &mut is_less);
        }

        runs.push(Run { start, len: end - start });
        end = start;

        while let Some(r) = collapse(&runs) {
            let left = runs[r + 1];
            let right = runs[r];
            unsafe {
                merge(
                    &mut v[left.start..right.start + right.len],
                    left.len,
                    buf.as_mut_ptr(),
                    &mut is_less,
                );
                merge_cost += 1;
            }
            runs[r] = Run { start: left.start, len: left.len + right.len };
            runs.remove(r + 1);
        }
    }

    debug_assert!(runs.len() == 1 && runs[0].start == 0 && runs[0].len == len);

    #[inline]
    fn collapse(runs: &[Run]) -> Option<usize> {
        let n = runs.len();
        if n >= 2
            && (runs[n - 1].start == 0
                || runs[n - 2].len <= runs[n - 1].len
                || (n >= 3 && runs[n - 3].len <= runs[n - 2].len + runs[n - 1].len)
                || (n >= 4 && runs[n - 4].len <= runs[n - 3].len + runs[n - 2].len))
        {
            if n >= 3 && runs[n - 3].len < runs[n - 1].len { Some(n - 3) } else { Some(n - 2) }
        } else {
            None
        }
    }

    #[derive(Clone, Copy)]
    struct Run {
        start: usize,
        len: usize,
    }

    merge_cost
}

fn benchmark_standardsort_1(c: &mut Criterion) {
    let mut is_less = |a: &i32, b: &i32| a < b;
    let mut group = c.benchmark_group("standard_powersort_runs");
    for size in [1000usize, 10000, 100000, 1000000, 100000000].into_iter() {
       for average_length_percentage in [0.01, 0.025, 0.05].into_iter() {
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
                tim_sort(&mut v, &mut is_less);
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
    for size in [1000usize, 10000, 100000].into_iter() {
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
                tim_sort(&mut v, &mut is_less)
            }
            , BatchSize::SmallInput);
        });
    }
    group.finish();
}

fn benchmark_standardsort_3(c: &mut Criterion) {
    let mut is_less = |a: &i32, b: &i32| a < b;
    let mut group = c.benchmark_group("standard_powersort_drag");
    for size in [1000usize, 10000, 100000].into_iter() {
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
                tim_sort(&mut v, &mut is_less)
            }
            , BatchSize::SmallInput);
        });
    }
    group.finish();
}

criterion_group!(standard_sort_group, benchmark_standardsort_2);
criterion_main!(standard_sort_group);