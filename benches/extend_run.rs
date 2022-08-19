use criterion::*;
use powersort::powersort::extend_run_right;

fn benchmark_extend_run_1(c: &mut Criterion) {
    
}

criterion_group!(extend_run_group, benchmark_extend_run_1);
criterion_main!(extend_run_group);
