use std::ptr;

pub fn insert_sort<T, F>(v: &mut [T], is_less: &mut F)
where
    F: FnMut(&T, &T) -> bool
{
    let first = unsafe { ptr::read(v.get_unchecked(0)) };
    // We find the correct position to insert `v[0]`.
    // Then we shift the sequence to make space for it and finally copy it
    // in the hole
    if v.len() >= 2 && is_less(&v[1], &first) {

        for i in (1..v.len()).rev() {
            if is_less(&v[i], &first) {
                // Shift v[1..i] to v[0..i-1] by copying
                unsafe {
                    ptr::copy_nonoverlapping(
                        v.get_unchecked(1),
                        v.get_unchecked_mut(0),
                        i,
                    );
                }

                // Insert first in the hole
                v[i] = first;
                break;
            }
        }
    }
}


/// Inserts `v[0]` into pre-sorted sequence `v[1..]` so that whole `v[..]` becomes sorted.
///
/// This is the integral subroutine of insertion sort.
#[cfg(not(no_global_oom_handling))]
pub fn insertion_sort<T, F>(v: &mut [T], mut is_less: F)
where
    F: FnMut(&T, &T) -> bool
{
    let n = v.len();
    for i in (0..n).rev() {
        insert_sort(&mut v[i..], &mut is_less);
    }
}

#[cfg(not(no_global_oom_handling))]
pub unsafe fn merge<T, F>(v: &mut [T], mid: usize, buf: *mut T, is_less: &mut F)
where
    F: FnMut(&T, &T) -> bool,
{
    use std::mem;

    let len = v.len();
    let v = v.as_mut_ptr();
    let (v_mid, v_end) = unsafe { (v.add(mid), v.add(len)) };

    let mut hole;

    if mid <= len - mid {
        // The left run is shorter.
        unsafe {
            ptr::copy_nonoverlapping(v, buf, mid);
            hole = MergeHole { start: buf, end: buf.add(mid), dest: v };
        }

        // Initially, these pointers point to the beginnings of their arrays.
        let left = &mut hole.start;
        let mut right = v_mid;
        let out = &mut hole.dest;

        while *left < hole.end && right < v_end {
            // Consume the lesser side.
            // If equal, prefer the left run to maintain stability.
            unsafe {
                let to_copy = if is_less(&*right, &**left) {
                    get_and_increment(&mut right)
                } else {
                    get_and_increment(left)
                };
                ptr::copy_nonoverlapping(to_copy, get_and_increment(out), 1);
            }
        }
    } else {
        // The right run is shorter.
        unsafe {
            ptr::copy_nonoverlapping(v_mid, buf, len - mid);
            hole = MergeHole { start: buf, end: buf.add(len - mid), dest: v_mid };
        }

        // Initially, these pointers point past the ends of their arrays.
        let left = &mut hole.dest;
        let right = &mut hole.end;
        let mut out = v_end;

        while v < *left && buf < *right {
            // Consume the greater side.
            // If equal, prefer the right run to maintain stability.
            unsafe {
                let to_copy = if is_less(&*right.offset(-1), &*left.offset(-1)) {
                    decrement_and_get(left)
                } else {
                    decrement_and_get(right)
                };
                ptr::copy_nonoverlapping(to_copy, decrement_and_get(&mut out), 1);
            }
        }
    }
    // Finally, `hole` gets dropped. If the shorter run was not fully consumed, whatever remains of
    // it will now be copied into the hole in `v`.

    unsafe fn get_and_increment<T>(ptr: &mut *mut T) -> *mut T {
        let old = *ptr;
        *ptr = unsafe { ptr.offset(1) };
        old
    }

    unsafe fn decrement_and_get<T>(ptr: &mut *mut T) -> *mut T {
        *ptr = unsafe { ptr.offset(-1) };
        *ptr
    }

    // When dropped, copies the range `start..end` into `dest..`.
    struct MergeHole<T> {
        start: *mut T,
        end: *mut T,
        dest: *mut T,
    }

    impl<T> Drop for MergeHole<T> {
        fn drop(&mut self) {
            // `T` is not a zero-sized type, and these are pointers into a slice's elements.
            unsafe {
                let len = (self.end as usize - self.start as usize) / mem::size_of::<T>();
                ptr::copy_nonoverlapping(self.start, self.dest, len);
            }
        }
    }
}

pub fn extend_run_left<T, F>(sequence: &[T], start: usize, is_less: &mut F) -> (usize, bool)
where
    F: FnMut(&T, &T) -> bool
{
    // If the start index is the first element, return 1.
    if start == 0 {
        return (1, true);
    }

    // All runs has at least one element.
    let mut length = 1;
    let mut i = start;

    // Keep track of whether the sequence is weakly increasing, we return it so we can reverse the run if necessary.
    let mut is_increasing = true;
    
    if is_less(&sequence[i], &sequence[i-1]) {
        is_increasing = false;
        // Strictly decreasing.
        while i > 0 && is_less(&sequence[i], &sequence[i-1]) {
            length += 1;
            i -= 1;
        }
    } else {
        // Weakly increasing.
        while i > 0 && !is_less(&sequence[i], &sequence[i-1]) {
            length += 1;
            i -= 1;
        }
    }

    (length, is_increasing)
}

/// This calculates the expected depth of the node in the nearly optimal binary merge tree.
pub fn node_power(s1: usize, n1: usize, n2: usize, n: usize) -> usize {
    let mut a = 2 * s1 + n1;
    let mut b = a + n1 + n2;
    let mut power = 0usize;

    loop {
        power += 1;
        if a >= n {
            a -= n;
            b -= n;
        } else if b >= n {
            break;
        }
        a <<= 1;
        b <<= 1;
    }

    power
}

fn capacity(n: usize) -> usize {
    n.next_power_of_two() + 1
}

/// This merge sort borrows some (but not all) ideas from TimSort, which is described in detail
/// [here](https://github.com/python/cpython/blob/main/Objects/listsort.txt).
///
/// The algorithm identifies strictly descending and non-descending subsequences, which are called
/// natural runs. There is a stack of pending runs yet to be merged. Each newly found run is pushed
/// onto the stack, and then some pairs of adjacent runs are merged until these two invariants are
/// satisfied:
///
/// 1. for every `i` in `1..runs.len()`: `runs[i - 1].len > runs[i].len`
/// 2. for every `i` in `2..runs.len()`: `runs[i - 2].len > runs[i - 1].len + runs[i].len`
///
/// The invariants ensure that the total running time is *O*(*n* \* log(*n*)) worst-case.
#[cfg(not(no_global_oom_handling))]
pub fn power_sort<T, F>(v: &mut [T], mut is_less: F)
where 
    F: FnMut(&T, &T) -> bool,
{
    use std::mem::size_of;

    // Runs less than this value are extended using insertion sort.
    const MIN_RUN_LENGTH: usize = 10;
    // Sequences less than this length are sorted using insertion sort.
    const MAX_INSERTION: usize = 20;

    if size_of::<T>() == 0 {
        return;
    }

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
        insert_sort(&mut v[e1-n1..=e1], &mut is_less);
        n1 += 1;
    }

    // Start of the run
    let mut s1 = e1 - (n1 - 1);

    // Buffer for merging runs
    let mut buf = Vec::with_capacity(n / 2);

    // Look for runs and merge if possible.
    while s1 > 0 {
        // Find second run.
        let e2 = s1 - 1;
        let (mut n2, is_increasing) = extend_run_left(v, e2, &mut is_less);

        if !is_increasing {
            v[e2-(n2-1)..=e2].reverse();
        }
        
        while n2 < MIN_RUN_LENGTH && e2-(n2-1) > 0 {
            insert_sort(&mut v[e2-n2..=e2], &mut is_less);
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
                unsafe { merge(&mut v[s1..=run.end], n1, buf.as_mut_ptr(), &mut is_less) };
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
        unsafe { merge(&mut v[s1..=run.end], n1, buf.as_mut_ptr(), &mut is_less) };
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
    // Test 50 runs.
    fn power_sort_test_3() {
        use crate::sequences::generate_m_runs;

        let mut v = generate_m_runs(500, 50);
        let mut sorted = v.clone();
        sorted.sort();

        power_sort(&mut v, |a, b| a < b);
        assert_eq!(v, sorted);
    }
}