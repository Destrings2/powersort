use std::vec;

use rand::prelude::*;
use rand_distr::{ChiSquared, Uniform};

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

fn generate_runs_with_average_length(length: usize, average_run_length: usize) -> Vec<i32> {
    let mut sequence = generate_random_sequence(length);
    let chi: ChiSquared<f64> = ChiSquared::new(average_run_length as f64).unwrap();

    let mut rng = thread_rng();

    let mut pointer = 0;
    let mut flip = false;
    while pointer < length {
        let run_length = chi.sample(&mut rng);
        let run_length = run_length as usize;
        let run_end = (pointer + run_length).min(length);

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

fn generate_runs_with_lengths(length: usize, lengths: &[usize]) -> Vec<i32> {
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

fn timsort_drag_run_lengths(n: usize) -> Vec<usize> {
    let mut lenghts;

    if n <= 3 {
        lenghts = vec![n];
    } else {
        let n_prime = n / 2;
        let last = n % 2 + 1;

        lenghts = timsort_drag_run_lengths(n_prime);
        lenghts.append(&mut timsort_drag_run_lengths(n_prime - 1));
        lenghts.push(last);
    }

    lenghts
}

fn generate_timsort_drag(length: usize, min_run_length: usize) -> Vec<i32> {
    let min_length = length / min_run_length;
    let mut lengths = timsort_drag_run_lengths(min_length);
    lengths = lengths.into_iter().map(| x | x * min_run_length).collect();

    generate_runs_with_lengths(length, &lengths)
}

fn generate_m_runs(length: usize, m: u32) -> Vec<i32> {
    let mut sequence = generate_random_sequence(length);
    let chi: ChiSquared<f64> = ChiSquared::new(m as f64).unwrap();

    let mut rng = thread_rng();

    let mut pointer = 0;
    let mut flip = false;
    for i in 0..m {
        let run_length = chi.sample(&mut rng);
        let run_length = run_length as usize;

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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_generate_random_sequence() {
        let sequence = generate_random_sequence(10);
        println!("{:?}", sequence);
        assert_eq!(sequence.len(), 10);
    }

    #[test]
    fn test_generate_random_sequence_in_range() {
        let sequence = generate_random_sequence_in_range(10, 0, 25);
        println!("{:?}", sequence);
        assert_eq!(sequence.len(), 10);
    }

    #[test]
    fn test_generate_runs() {
        let sequence = generate_runs_with_average_length(20, 5);
        println!("{:?}", sequence);
        assert_eq!(sequence.len(), 20);
    }

    #[test]
    fn test_generate_runs_2() { 
        let sequence = generate_m_runs(20, 10);
        println!("{:?}", sequence);
        assert_eq!(sequence.len(), 20);
    }

    #[test]
    fn test_timsort_drag_run_lengths() {
        let lengths = timsort_drag_run_lengths(54);
        let expected = vec![3, 2, 1, 2, 1, 2, 2, 3, 2, 1, 2, 1, 2, 1, 2, 3, 2, 1, 2, 1, 2, 2, 3, 2, 1, 2, 1, 2, 1, 1, 1];
        assert_eq!(lengths, expected);
    }

    #[test]
    fn test_generate_timsort_drag() {
        let sequence = generate_timsort_drag(30, 5);
        println!("{:?}", sequence);
    }
}