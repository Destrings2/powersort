use std::vec;

use rand::prelude::*;
use rand_distr::{ChiSquared, Uniform};

/// Generates a random vector of size `length` with elements in the range `[0, length)`.
/// The elements are generated using a uniform distribution.
/// # Arguments
/// - `length`: The length of the vector to generate.
/// # Returns
/// A vector of length `length` with elements in the range `[0, length)`.
pub fn generate_random_sequence(length: usize) -> Vec<i32> {
    generate_random_sequence_in_range(length, 0, length as i32)
}

/// See [generate_random_sequence]. This function allows you to specify the range of the elements to generate.
/// # Arguments
/// - `length`: The length of the vector to generate.
/// - `low`: The lowest possible value of the elements.
/// - `high`: The highest possible value of the elements.
/// # Returns
/// A vector of length `length` with elements in the range `[low, high)`.
pub fn generate_random_sequence_in_range(length: usize, low: i32, high: i32) -> Vec<i32>
{
    // Use a uniform distribution to generate the elements.
    let distribution = Uniform::new_inclusive(low, high);
    let rng = thread_rng();

    // Sample the distribution `length` times and collect the results into a vector.
    rng.sample_iter(distribution)
        .take(length)
        .collect()
}

/// Generates a random vector of size `length` containing *runs* of elements in the range `[0, length)`.
/// A *run* is a sorted sequence of elements.
/// The elements are generated using a uniform distribution. The length of
/// each run is generated using a chi-squared distribution.
/// # Arguments
/// - `length`: The length of the vector to generate.
/// - `average_run_length`: The average length of each run.
/// # Returns
/// A vector of length `length` containing *runs* of elements in the range `[0, length)`.
pub fn generate_runs_with_average_length(length: usize, average_run_length: usize) -> Vec<i32> {
    // Generate a random sequence to be sorted in runs.
    let mut sequence = generate_random_sequence(length);

    // The chi-squared distribution has a parameter that is the number of degrees of freedom.
    // This parameter coincides with the mean of the distribution, which represents the average length of a run.
    let chi: ChiSquared<f64> = ChiSquared::new(average_run_length as f64).unwrap();

    let mut rng = thread_rng();

    let mut pointer = 0;
    // Runs alternate between ascending and descending so that no two runs form a single run by chance.
    let mut flip = false;
    while pointer < length {
        let run_length = chi.sample(&mut rng);
        let run_length = run_length as usize;

        // The end of the run can't be past the end of the sequence.
        let run_end = (pointer + run_length).min(length);

        // Reference a slice of the sequence from the current position to the end of the run and sort it.
        let run = &mut sequence[pointer..run_end];
        run.sort();

        if flip {
            run.reverse();
            flip = false;
        } else {
            flip = true;
        }

        pointer = run_end;
    }

    sequence
}


/// Instead of generating runs with an average length, this function generates runs with a fixed length.
/// See [generate_runs_with_average_length].
/// # Arguments
/// - `length`: The length of the vector to generate.
/// - `lengths`: The lengths of the runs to generate.
/// # Returns
/// A vector of length `length` containing *runs* of elements in the range `[0, length)`.
pub fn generate_runs_with_lengths(length: usize, lengths: &[usize]) -> Vec<i32> {
    // Make sure the lengths total the length of the sequence.
    assert_eq!(lengths.iter().sum::<usize>(), length);
    let mut sequence = generate_random_sequence(length);

    let mut pointer = 0;
    let mut flip = false;
    for &run_length in lengths {
        let run_end = pointer + run_length;

        let run = &mut sequence[pointer..run_end];
        run.sort();
        if flip {
            run.reverse();
            flip = false;
        } else {
            flip = true;
        }

        pointer = run_end;
    }

    sequence
}

/// Generates the run lenghts for a given size. These run lenghts
/// are such that TimSort performs suboptimally when sorting the runs.
/// See Sam Buss & Alexander Knop (2018). Strategies for Stable Merge Sorting.
fn get_timsort_drag_run_lengths(n: usize) -> Vec<usize> {
    let mut lenghts;

    if n <= 3 {
        lenghts = vec![n];
    } else {
        let n_prime = n / 2;
        let last = n % 2 + 1;

        lenghts = get_timsort_drag_run_lengths(n_prime);
        lenghts.append(&mut get_timsort_drag_run_lengths(n_prime - 1));
        lenghts.push(last);
    }

    lenghts
}

/// Generates a sequence that makes TimSort perform suboptimally. 
/// This is useful for testing the performance of the algorithm.
/// # Arguments
/// - `length`: The length of the vector to generate.
/// - `min_run_length`: The minimum length of a run.
/// # Returns
/// A vector of length `length` containing runs of elements in the range `[0, length)`.
pub fn generate_timsort_drag(length: usize, min_run_length: usize) -> Vec<i32> {
    let min_length = length / min_run_length;
    let mut lengths = get_timsort_drag_run_lengths(min_length);

    // The lenghts are multiplied by the minimum run length.
    lengths = lengths.into_iter().map(| x | x * min_run_length).collect();

    generate_runs_with_lengths(length, &lengths)
}

/// Generates at most `m` runs for a sequence of length `length`
/// See [generate_runs_with_average_length].
/// # Arguments
/// - `length`: The length of the vector to generate.
/// - `m`: The number of runs to generate.
/// # Returns
/// A vector of length `length` containing `m` *runs* of elements in the range `[0, length)`.
pub fn generate_m_runs(length: usize, m: u32) -> Vec<i32> {
    let mut sequence = generate_random_sequence(length);

    // The average length of a run is the length of the sequence divided by the number of runs.
    let chi: ChiSquared<f64> = ChiSquared::new(length as f64 / m as f64).unwrap();

    let mut rng = thread_rng();

    let mut pointer = 0;
    let mut flip = false;
    for i in 0..m {
        let run_length = chi.sample(&mut rng);
        let run_length = run_length as usize;

        // If we are generating the last run, make sure the end is at the end of the sequence.
        let run_end = if i == m {
            length
        } else {
            (pointer + run_length).min(length)
        };

        let run = &mut sequence[pointer..run_end];
        run.sort();
        if flip {
            run.reverse();
            flip = false;
        } else {
            flip = true;
        }

        pointer = run_end;
    }

    sequence
}