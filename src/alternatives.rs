// Alternatives for the powersort functions.

use std::ptr;

/// Instead of inserting `v[n-1]` into `v`, it inserts `v[0]` into v.
/// This way the run can grow from right to left.
pub fn insert_sort_left<T, F>(v: &mut [T], is_less: &mut F)
where
    F: FnMut(&T, &T) -> bool,
    T: Copy
{
    let first = v[0];
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

#[cfg(test)]
mod insert_sort_left_tests {
    use super::insert_sort_left;

    #[test]
    fn insert_sort_test_1() {
        let mut is_less = |a: &i32, b: &i32| a < b;
        let mut v = vec![5, 4, 2, 3, 6, 7];

        insert_sort_left(&mut v[1..], &mut is_less);
        insert_sort_left(&mut v[0..], &mut is_less);

        assert_eq!(v, vec![2, 3, 4, 5, 6, 7]);
    }
    
    #[test]
    fn insert_sort_test_2() {
        let mut is_less = |a: &i32, b: &i32| a < b;
        let mut v = vec![3, 2];

        insert_sort_left(&mut v, &mut is_less);

        assert_eq!(v, vec![2, 3]);
    }
}


/// Like [extend_run_right], but the run is growing from right to left.
/// See [extend_run_right].
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

#[cfg(test)]
mod extend_run_left_tests {
    use super::extend_run_left;

    #[test]
    // Test on a weakly increasing sequence.
    fn extend_run_left_1() {
        let sequence = [1, 1, 2, 3, 4, 4, 5, 6, 7, 8, 9, 10];
        let mut is_less = |a: &i32, b: &i32| a < b;
        let (length, is_increasing) = extend_run_left(&sequence, sequence.len() - 1, &mut is_less);
        assert_eq!(length, 12);
        assert!(is_increasing);
    }

    #[test]
    // Test on a strictly decreasing sequence.
    fn extend_run_left_2() {
        let sequence = [10, 9, 8, 7, 6, 5, 4, 3, 2, 1];
        let mut is_less = |a: &i32, b: &i32| a < b;
        let (length, is_increasing) = extend_run_left(&sequence, sequence.len() - 1, &mut is_less);
        assert_eq!(length, 10);
        assert!(!is_increasing);
    }

    #[test]
    // Test on a mixed sequence.
    fn extend_run_left_3() {
        let sequence = [1, 1, 2, 3, 4, 4, 6, 5, 4, 3, 2, 1];
        let mut is_less = |a: &i32, b: &i32| a < b;
        let (length, is_increasing) = extend_run_left(&sequence, sequence.len() - 1, &mut is_less);
        assert_eq!(length, 6);
        assert!(!is_increasing);

        let (length, is_increasing) = extend_run_left(&sequence, 5, &mut is_less);
        assert_eq!(length, 6);
        assert!(is_increasing);
    }

    #[test]
    // Edge case: If the start index is the first element, return 1.
    fn extend_run_left_4() {
        let sequence = [1, 1, 2, 3, 4, 4, 5, 6, 7, 8, 9, 10];
        let mut is_less = |a: &i32, b: &i32| a < b;
        let (length, is_increasing) = extend_run_left(&sequence, 0, &mut is_less);
        assert_eq!(length, 1);
        assert!(is_increasing);
    }

    #[test]
    fn extend_run_left_5() {
        let sequence = vec![5, 7, 9, 1, 2, 4, 6];
        let mut is_less = |a: &i32, b: &i32| a < b;

        let (length, is_increasing) = extend_run_left(&sequence, sequence.len() - 1, &mut is_less);
        assert_eq!(length, 4);
        assert!(is_increasing);

        let (length, is_increasing) = extend_run_left(&sequence, 2, &mut is_less);
        assert_eq!(length, 3);
        assert!(is_increasing);
    }
}

/// Like [merge] but accepts the buffer as a parameter. This is theory should save on allocations.
/// # Safety
/// The buffer must be large enough to hold all the elements in the sequence.
pub unsafe fn merge_buffer_reuse<T, F>(v: &mut [T], mid: usize, buf: *mut T, is_less: &mut F)
where
    F: FnMut(&T, &T) -> bool,
    T: Copy
{
    // This is the initial naive implementation of merge.
    let n = v.len();

    // Copy the sequence so we can modify the original sequence.
    ptr::copy_nonoverlapping(v.get_unchecked(0), buf, n);

    let mut i = 0;
    let mut j = mid + 1;
    let mut k = 0;

    // While both runs are not consumed
    while i <= mid && j < n {
        // Copy the next element according to the relative ordering.
        if is_less(&*buf.add(i), &*buf.add(j)) {
            v[k] = *buf.add(i);
            i += 1;
        } else {
            v[k] = *buf.add(j);
            j += 1;
        }
        k += 1;
    }

    // If the first run is not consumed, copy the remaining elements.
    while i <= mid {
        v[k] = *buf.add(i);
        i += 1;
        k += 1;
    }

    // If the second run is not consumed, copy the remaining elements.
    while j < n {
        v[k] = *buf.add(j);
        j += 1;
        k += 1;
    }
}

pub fn node_power_no_div(s1: usize, n1: usize, n2: usize, n: usize) -> u32 {
    let mut a = 2 * s1 + n1;
    let mut b = a + n1 + n2;
    let mut power = 0u32;

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