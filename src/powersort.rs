use std::ptr;

/// Given a sequence, and a start index, returns the number of elements that are strictly decreasing, 
/// or weakly increasing, from the star&t index to the end of the sequence.
/// # Arguments
/// - `sequence`: The sequence to search.
/// - `start`: The index to start searching from.
/// - `is_less`: A function that returns true if the first element is less than the second element.
/// # Returns
/// - The number of elements that are strictly decreasing, or weakly increasing, from the start index to the end of the sequence.
/// - Whether the sequence is weakly increasing from the start index to the end of the sequence.
pub fn extend_run_right<T, F>(sequence: &[T], start: usize, is_less: &mut F) -> (usize, bool)
where
    F: FnMut(&T, &T) -> bool
{
    // If the start index is the last element, return 1.
    if start == sequence.len() - 1 {
        return (1, true);
    }

    // All runs has at least one element.
    let mut length = 1;
    let mut i = start + 1;

    // Keep track of whether the sequence is weakly increasing, we return it so we can reverse the run if necessary.
    let mut is_increasing = true;
    
    if is_less(&sequence[i], &sequence[i-1]) {
        is_increasing = false;
        // Strictly decreasing.
        while i < sequence.len() && is_less(&sequence[i], &sequence[i-1]) {
            length += 1;
            i += 1;
        }
    } else {
        // Weakly increasing.
        while i < sequence.len() && !is_less(&sequence[i], &sequence[i-1]) {
            length += 1;
            i += 1;
        }
    }

    (length, is_increasing)
}

#[cfg(test)]
mod extend_run_right_tests {
    use super::extend_run_right;

    #[test]
    // Test on a weakly increasing sequence.
    fn extend_run_right_1() {
        let sequence = [1, 1, 2, 3, 4, 4, 5, 6, 7, 8, 9, 10];
        let mut is_less = |a: &i32, b: &i32| a < b;
        let (length, is_increasing) = extend_run_right(&sequence, 0, &mut is_less);
        assert_eq!(length, 12);
        assert!(is_increasing);
    }

    #[test]
    // Test on a strictly decreasing sequence.
    fn extend_run_right_2() {
        let sequence = [10, 9, 8, 7, 6, 5, 4, 3, 2, 1];
        let mut is_less = |a: &i32, b: &i32| a < b;
        let (length, is_increasing) = extend_run_right(&sequence, 0, &mut is_less);
        assert_eq!(length, 10);
        assert!(!is_increasing);
    }

    #[test]
    // Test on a mixed sequence.
    fn extend_run_right_3() {
        let sequence = [1, 1, 2, 3, 4, 4, 6, 5, 4, 3, 2, 1];
        let mut is_less = |a: &i32, b: &i32| a < b;
        let (length, is_increasing) = extend_run_right(&sequence, 0, &mut is_less);
        assert_eq!(length, 7);
        assert!(is_increasing);

        let (length, is_increasing) = extend_run_right(&sequence, 7, &mut is_less);
        assert_eq!(length, 5);
        assert!(!is_increasing);
    }

    #[test]
    // Edge case: If the start index is the last element, return 1.
    fn extend_run_right_4() {
        let sequence = [1, 1, 2, 3, 4, 4, 5, 6, 7, 8, 9, 10];
        let mut is_less = |a: &i32, b: &i32| a < b;
        let (length, is_increasing) = extend_run_right(&sequence, sequence.len() - 1, &mut is_less);
        assert_eq!(length, 1);
        assert!(is_increasing);
    }
}

/// Computes the power between two runs. The power is the expected depth in a binary merge tree.
/// This is used to determine a nearly optimal merge order for PowerSort.
/// This function expects runs to be next to each other in the sequence, which is the case
/// for the runs used in PowerSort.
/// # Arguments
/// - `s1`: The start index of the first run.
/// - `n1`: The length of the first run.
/// - `n2`: The length of the second run.
/// - `n`: The length of the entire sequence.
pub fn node_power(s1: usize, n1: usize, n2: usize, n: usize) -> usize {
    // Theoretically, a and b are numbers between [0,1) and represent the position of the midpoints of the runs
    // in the sequence as a fraction of the total length of the sequence.
    // The expected depth is the first bit where the (binary) fractional parts of a and b differ.
    // See J. Ian Munro & Sebastian Wild (2018). Nearly-Optimal Mergesorts: Fast, Practical Sorting Methods That Optimally Adapt to Existing Runs.
    // 
    // We use algebraic manipulation to convert the alculation into the following forms.
    // We are originally given s1, e1, s2, e2, and n as the start index of the first run,
    // the length of the first run, the start index of the second run, the
    // length of the second run and the length of the entire sequence respectively.
    //
    // Then the paper defines
    // n1 = e1 - s1 + 1
    // n2 = e2 - s2 + 1
    // a = (s1 + n1/2 - 1)/n
    // b = (s2 + n2/2 - 1)/n
    //
    // If we assume the runs are next to each other in the sequence, and are given s1, n1, n2, and n then we can
    // rewrite the above formula as follows:
    // a = 2(s1 + n1/2 - 1)/2n
    // => (2*s1 + n1 - 2)/2n
    // b = 2((s1 + n1 + 1) + n2/2 - 1)/2n
    // => (2*s1 + 2*n1 + n2)/2n
    // => (a + n1 + n2 + 2)/2n
    //
    // Then we need the first bit for which the fractional parts of a and b differ, that is
    // the minimum l for which floor(a/2n*2^l) != floor(b/2n*2^l).
    // => floor(a*2^l-1/n) != floor(b*2^l-1/n)

    let mut a = 2*s1 + n1 - 2;
    let mut b = a + n1 + n2 + 2;

    let mut power = 0;

    while a/n == b/n {
        power += 1;
        a <<= 1;
        b <<= 1;
    }

    // Adjust + 1, it's not really necessary as we are interested in the relative ordering, but
    // helps the comparison with manually computed values in the unit tests.
    power + 1
}

#[cfg(test)]
mod node_power_tests {
    use super::node_power;

    // Test with manually computed values.
    #[test]
    fn node_power_test_1() {
        assert_eq!(node_power(11, 6, 1, 20), 2);
    }

    #[test]
    fn node_power_test_2() {
        assert_eq!(node_power(44, 12, 3, 60), 3);
    }

    #[test]
    fn node_power_test_3() {
        assert_eq!(node_power(0, 11, 11, 21), 1);
    }
}

/// Computes the required vector capacity for the stack used in PowerSort.
pub fn capacity(n: usize) -> usize {
    (n as f64).log2() as usize + 1
}

/// For sequence `v`, merges `v[..mid]` and `v[mid..]` following a is_less comparison function.
/// The result is stored in the original vector.
/// # Arguments
/// - `v`: The vector to merge.
/// - `mid`: The index of the middle element.
/// - `is_less`: The comparison function.
/// # Panics
/// Panics if `mid` is out of bounds, or is_less panics for a given comparison.
pub fn merge<T, F>(v: &mut [T], mid: usize, is_less: &mut F)
where
    F: FnMut(&T, &T) -> bool,
    T: Copy
{
    // This is the initial naive implementation of merge.
    let n = v.len();

    // Copy the sequence so we can modify the original sequence.
    let copy = v.to_vec();

    let mut i = 0;
    let mut j = mid + 1;
    let mut k = 0;

    // While both runs are not consumed
    while i <= mid && j < n {
        // Copy the next element according to the relative ordering.
        if is_less(&copy[i], &copy[j]) {
            v[k] = copy[i];
            i += 1;
        } else {
            v[k] = copy[j];
            j += 1;
        }
        k += 1;
    }

    // If the first run is not consumed, copy the remaining elements.
    while i <= mid {
        v[k] = copy[i];
        i += 1;
        k += 1;
    }

    // If the second run is not consumed, copy the remaining elements.
    while j < n {
        v[k] = copy[j];
        j += 1;
        k += 1;
    }
}

#[cfg(test)]
mod merge_tests {
    use super::merge;

    #[test]
    fn merge_test_1() {
        let mut v = vec![2, 3, 6, 7, 1, 4, 5];
        merge(&mut v, 3, &mut |a, b| a < b);
        assert_eq!(v, vec![1, 2, 3, 4, 5, 6, 7]);
    }

    #[test]
    fn merge_test_2() {
        let mut v = vec![5, 6, 7, 8, 9, 0, 1, 2, 3, 4];
        merge(&mut v, 4, &mut |a, b| a < b);
        assert_eq!(v, vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
    }


    #[test]
    fn merge_test_3() {
        let mut v = vec![1, 2, 3, 4, 5, 6];
        merge(&mut v, 2, &mut |a, b| a < b);
        assert_eq!(v, vec![1, 2, 3, 4, 5, 6]);
    }
}

/// Inserts `v[0]` into the presorted sequence `v` so that the whole `v[..]` is sorted
/// This is useful for extending runs.
/// # Arguments
/// v: slice of presorted elements for which the last element is not
/// is_less: comparison function
pub fn insert_sort<T, F>(v: &mut [T], is_less: &mut F)
where
    F: FnMut(&T, &T) -> bool,
    T: Copy
{
    // We find the correct position to insert `v[0]`.
    // Then we shift the sequence to make space for it and finally copy it
    // in the hole
    let n = v.len();
    let last = v[n-1];

    if v.len() >= 2 && is_less(&last, &v[n-2]) {
        let n = v.len();

        for i in 0..n-1 {
            if is_less(&last, &v[i]) {
                // Shift v[i..n-1] to v[i+1..n]
                unsafe {
                    ptr::copy_nonoverlapping(
                        v.get_unchecked(i), 
                        v.get_unchecked_mut(i + 1), 
                        n-i-1
                    );
                }

                // Copy the new element in the hole.
                v[i] = last;
                break;
            }
        }
    }
}

#[cfg(test)]
mod insert_sort_test {
    use super::insert_sort;

    #[test]
    fn insert_sort_test_1() {
        let mut is_less = |a: &i32, b: &i32| a < b;
        let mut v = vec![2, 3, 6, 7, 5, 4];

        insert_sort(&mut v[..5], &mut is_less);
        insert_sort(&mut v[..6], &mut is_less);

        assert_eq!(v, vec![2, 3, 4, 5, 6, 7]);
    }
    
    #[test]
    fn insert_sort_test_2() {
        let mut is_less = |a: &i32, b: &i32| a < b;
        let mut v = vec![3, 2];

        insert_sort(&mut v, &mut is_less);

        assert_eq!(v, vec![2, 3]);
    }
}


/// Sorts the sequence `v` using insertion sort.
/// While insertion sort is O(n^2), it is faster than merge sort for small sequences.
/// # Arguments
/// - `v`: The sequence to sort.
/// - `is_less`: The comparison function.
pub fn insertion_sort<T, F>(v: &mut [T], mut is_less: F)
where
    F: FnMut(&T, &T) -> bool,
    T: Copy
{
    let n = v.len();
    for i in 1..=n {
        insert_sort(&mut v[..i], &mut is_less);
    }
}

/// Sorts the sequence `v` using PowerSort.
/// # Arguments
/// - `v`: The sequence to sort.
/// - `is_less`: The comparison function.
pub fn power_sort<T, F>(v: &mut [T], mut is_less: F)
where 
    F: FnMut(&T, &T) -> bool,
    T: Copy
{
    // Runs less than this value are extended using insertion sort.
    const MIN_RUN_LENGTH: usize = 10;
    // Sequences less than this length are sorted using insertion sort.
    const MAX_INSERTION: usize = 20;

    let n = v.len();
    
    // Use insertion sort for small sequences as it is faster.
    if n < MAX_INSERTION {
        insertion_sort(v, &mut is_less);
        return;
    }

    // Stack for storing runs.
    let mut runs: Vec<Run> = Vec::with_capacity(capacity(n));

    let mut s1 = 0;
    let (mut n1, is_increasing) = extend_run_right(v, s1, &mut is_less);

    // Reverse the run if it is decreasing so we only have (weakly) increasing runs.
    if !is_increasing {
        v[s1..s1+n1].reverse();
    }

    // Extend the first run to the left until it is long enough.
    while n1 < MIN_RUN_LENGTH && s1+n1 < n {
        insert_sort(&mut v[s1..s1+n1+1], &mut is_less);
        n1 += 1;
    }

    // Look for runs and merge if possible.
    while s1 + n1 < n {
        // Find second run.
        let s2 = s1 + n1;
        let (mut n2, is_increasing) = extend_run_right(v, s2, &mut is_less);

        if !is_increasing {
            v[s2..s2+n2].reverse();
        }
        
        while n1 < MIN_RUN_LENGTH && s2+n2 < n {
            insert_sort(&mut v[s2..s2+n2+1], &mut is_less);
            n2 += 1;
        }

        // Compute power between runs.
        let power = node_power(s1, n1, n2, n);

        // Merge if possible.
        while let Some(run) = runs.last() {
            // If the top run's power is not greater than the power of the new run, merge them.
            if run.power > power {
                let run = runs.pop().unwrap();

                // Merge the two runs.
                merge(&mut v[run.start..s1+n1], run.length-1, &mut is_less);
                s1 = run.start;
                n1 += run.length;
            } else {
                // Else, exit.
                break;
            }
        }

        // Push the new run.
        runs.push(Run {
            start: s1,
            length: n1,
            power
        });

        s1 = s2;
        n1 = n2;
    }

    // Merge remaining runs.
    while let Some(run) = runs.pop() {
        merge(&mut v[run.start..s1+n1], run.length-1, &mut is_less);
        s1 = run.start;
        n1 += run.length;
    }


    /// Represents a run of elements in a vector.
    #[derive(Clone, Copy, Debug)]
    struct Run {
        start: usize,
        length: usize,
        power: usize
    }
}

pub fn sort<T>(v: &mut [T])
where
    T: Copy + Ord
{
    power_sort(v,  |a, b| a < b);
}

#[cfg(test)]
mod powesort_tests {
    use super::power_sort;

    #[test]
    // Test for two runs.
    fn power_sort_test_1() {
        let mut v = vec![2, 3, 6, 7, 1, 4, 5];
        power_sort(&mut v, |a, b| a < b);
        assert_eq!(v, vec![1, 2, 3, 4, 5, 6, 7]);
    }

    #[test]
    // Test for three runs.
    fn power_sort_test_2() {
        let mut v = vec![5, 6, 7, 10, 9, 8, 1, 2, 3, 4];
        power_sort(&mut v, |a, b| a < b);
        assert_eq!(v, vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
    }

    #[test]
    // Test 500 runs.
    fn power_sort_test_3() {
        use crate::sequences::generate_m_runs;

        let mut v = generate_m_runs(500, 50);
        let mut sorted = v.clone();
        sorted.sort();

        power_sort(&mut v, |a, b| a < b);
        assert_eq!(v, sorted);
    }
}