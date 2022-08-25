use crate::{powersort::{capacity, extend_run_right, node_power, merge, insert_sort, insertion_sort}, alternatives::{extend_run_left, insert_sort_left, merge_buffer_reuse}};

pub fn power_sort_no_insertion<T, F>(v: &mut [T], mut is_less: F)
where 
    F: FnMut(&T, &T) -> bool,
    T: Copy
{
    let n = v.len();

    // Stack for storing runs.
    let mut runs: Vec<Run> = Vec::with_capacity(capacity(n));

    let mut s1 = 0;
    let (mut n1, is_increasing) = extend_run_right(v, s1, &mut is_less);

    // Reverse the run if it is decreasing so we only have (weakly) increasing runs.
    if !is_increasing {
        v[s1..s1+n1].reverse();
    }

    // Look for runs and merge if possible.
    while s1 + n1 < n {
        let s2 = s1 + n1;
        let (mut n2, is_increasing) = extend_run_right(v, s2, &mut is_less);

        if !is_increasing {
            v[s2..s2+n2].reverse();
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

pub fn power_sort_left<T, F>(v: &mut [T], mut is_less: F)
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

    let mut e1 = n-1;
    let (mut n1, is_increasing) = extend_run_left(v, e1, &mut is_less);
    // Reverse the run if it is decreasing so we only have (weakly) increasing runs.
    if !is_increasing {
        v[e1-(n1-1)..=e1].reverse();
    }

    // Extend the first run to the left until it is long enough.
    while n1 < MIN_RUN_LENGTH && e1-(n1-1) > 0  {
        insert_sort_left(&mut v[e1-n1..=e1], &mut is_less);
        n1 += 1;
    }

    // Start of the run
    let mut s1 = e1 - (n1 - 1);

    // Look for runs and merge if possible.
    while s1 > 0 {
        // Find second run.
        let e2 = s1 - 1;
        let (mut n2, is_increasing) = extend_run_left(v, e2, &mut is_less);

        if !is_increasing {
            v[e2-(n2-1)..=e2].reverse();
        }
        
        while n2 < MIN_RUN_LENGTH && e2-(n2-1) > 0 {
            insert_sort_left(&mut v[e2-n2..=e2], &mut is_less);
            n2 += 1;
        }

        let s2 = e2 - (n2 - 1);

        // Compute power between runs.
        let power = node_power(s1, n1, n2, n);

        // Merge if possible.
        while let Some(run) = runs.last() {
            // If the top run's power is not greater than the power of the new run, merge them.
            if run.power > power {
                let run = runs.pop().unwrap();

                // Merge the two runs.
                merge(&mut v[s1..=run.end], n1-1, &mut is_less);
                e1 = run.end;
                n1 += run.length;
                s1 = e1 - (n1 - 1);
            } else {
                // Else, exit
                break;
            }
        }

        // Push the new run.
        runs.push(Run {
            end: e1,
            length: n1,
            power
        });

        s1 = s2; 
        e1 = e2;
        n1 = n2;
    }

    // Merge remaining runs.
    while let Some(run) = runs.pop() {
        merge(&mut v[s1..=run.end], n1-1, &mut is_less);
        e1 = run.end;
        n1 += run.length;
        s1 = e1 - (n1 - 1);
    }


    /// Represents a run of elements in a vector.
    #[derive(Clone, Copy, Debug)]
    struct Run {
        end: usize,
        length: usize,
        power: usize
    }
}

#[cfg(test)]
mod power_sort_left_tests {
    use super::power_sort_left;

    #[test]
    fn power_sort_left_test_1() {
        let mut v = vec![5, 7, 9, 1, 2, 4, 8, 6];
        let mut is_less = |a: &i32, b: &i32| a < b;
        power_sort_left(&mut v, &mut is_less);
        assert_eq!(v, vec![1, 2, 4, 5, 6, 7, 8, 9]);
    }

    #[test]
    // Test 50 runs.
    fn power_sort_left_test_2() {
        use crate::sequences::generate_m_runs;

        let mut v = generate_m_runs(50, 10);
        let mut sorted = v.clone();
        sorted.sort();

        power_sort_left(&mut v, |a, b| a < b);
        assert_eq!(v, sorted);
    }
}

pub fn power_sort_buffer<T, F>(v: &mut [T], mut is_less: F)
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

    let mut buf: Vec<T> = Vec::with_capacity(n);

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
                unsafe {
                    merge_buffer_reuse(&mut v[run.start..s1+n1], run.length-1, buf.as_mut_ptr(),&mut is_less);
                }
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
        unsafe {
            merge_buffer_reuse(&mut v[run.start..s1+n1], run.length-1, buf.as_mut_ptr(), &mut is_less);
        }
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

#[cfg(test)]
mod power_sort_buffer_test {
    use super::power_sort_buffer;

    #[test]
    fn power_sort_left_test_1() {
        use crate::sequences::generate_m_runs;

        let mut v = generate_m_runs(50, 10);
        let mut sorted = v.clone();
        sorted.sort();

        power_sort_buffer(&mut v, |a, b| a < b);
        assert_eq!(v, sorted);
    } 
}