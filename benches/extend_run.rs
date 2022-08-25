use criterion::*;
use powersort::powersort::extend_run_right;

fn benchmark_extend_run_1(c: &mut Criterion) {
    let mut is_less = |a: &i32, b: &i32| a < b;

    let mut group = c.benchmark_group("extend_run_right");

    for size in [10usize, 100, 1000, 10000, 100000, 200000, 300000].into_iter() {
        // Create a vector of 0 to size - 1
        let v = (0i32..size as i32).into_iter().collect::<Vec<_>>();
        let mut v_reverse = v.clone();
        v_reverse.reverse();

        group.bench_function(BenchmarkId::new(
            "extend_run_right", format!("ascending_{}", size)),
            |b| {
            b.iter(|| {
                extend_run_right(&v, 0, &mut is_less);
            });
        });

        group.bench_function(BenchmarkId::new("extend_run_right", format!("descending_{}", size)),
        |b| {
            b.iter(|| {
                extend_run_right(&v_reverse, 0, &mut is_less);
            });
        });
    }

    group.finish();
}

criterion_group!(extend_run_group, benchmark_extend_run_1);
criterion_main!(extend_run_group);
