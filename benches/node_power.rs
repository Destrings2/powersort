use criterion::*;
use powersort::{powersort::node_power, alternatives::node_power_no_div};
use rand::Rng;

fn benchmark_node_power_1(c: &mut Criterion) {
    let mut rng = rand::thread_rng();

    let mut group = c.benchmark_group("node_power");

    let n = 100000;
    let n1 = rng.gen_range(0..=n/2);
    let n2 = rng.gen_range(0..=n/2);
    let s1 = n - n1 - n2;

    group.bench_function("node_power", |b| {
        b.iter(|| {
            node_power(s1, n1, n2, n);
        })
    });

    group.bench_function("node_power_no_div", |b| {
        b.iter(|| {
            node_power_no_div(s1, n1, n2, n);
        })
    });
}

criterion_group!(node_power_group, benchmark_node_power_1);
criterion_main!(node_power_group);