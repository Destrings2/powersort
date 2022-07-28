use std::ops::Range;

use rand::{distributions::Uniform, thread_rng, Rng};

fn generate_random_sequence(length: usize) -> Vec<i32> {
    generate_random_sequence_in_range(length, 0, length as i32)
}

fn generate_random_sequence_in_range(length: usize, low: i32, high: i32) -> Vec<i32>
{
    let distribution = Uniform::new_inclusive(low, high);
    let rng = thread_rng();

    rng.sample_iter(distribution)
        .take(length)
        .collect()
}

fn generate_runs(length: usize, runs: usize) {
    assert!(runs <= length, "Can't have more runs than length.");
    let mut sequence = generate_random_sequence(length);
}