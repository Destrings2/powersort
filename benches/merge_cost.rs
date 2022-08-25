use core::mem::size_of;
use std::{fs::File, io::Write};

use powersort::{sort::*, sequences::generate_m_runs};

pub fn power_sort<T, F>(v: &mut [T], mut is_less: F) -> usize
where
    F: FnMut(&T, &T) -> bool,
{
    let mut merge_cost = 0;
    const MAX_INSERTION: usize = 20;
    const MIN_RUN: usize = 10;

    if size_of::<T>() == 0 {
        return 0;
    }

    let len = v.len();

    if len <= MAX_INSERTION {
        if len >= 2 {
            for i in (0..len - 1).rev() {
                insert_head(&mut v[i..], &mut is_less);
            }
        }
        return 0;
    }

    let mut buf = Vec::with_capacity(len / 2);

    let mut runs: Vec<Run> = vec![];
    let mut end = len;
    while end > 0 {
        let mut start = end - 1;
        if start > 0 {
            start -= 1;
            unsafe {
                if is_less(v.get_unchecked(start + 1), v.get_unchecked(start)) {
                    while start > 0 && is_less(v.get_unchecked(start), v.get_unchecked(start - 1)) {
                        start -= 1;
                    }
                    v[start..end].reverse();
                } else {
                    while start > 0 && !is_less(v.get_unchecked(start), v.get_unchecked(start - 1))
                    {
                        start -= 1;
                    }
                }
            }
        }

        while start > 0 && end - start < MIN_RUN {
            start -= 1;
            insert_head(&mut v[start..end], &mut is_less);
        }

        let power = runs.last().map_or(0, |top|
            node_power(top.start, top.len, end - start, len)
        );

        runs.push(Run { start, len: end - start, power });
        end = start;

        while let Some(r) = collapse(&runs) {
            let left = runs[r + 1];
            let right = runs[r];
            unsafe {
                merge(
                    &mut v[left.start..right.start + right.len],
                    left.len,
                    buf.as_mut_ptr(),
                    &mut is_less,
                );
                merge_cost += left.len + right.len;
            }
            runs[r] = Run { start: left.start, len: left.len + right.len, power: right.power };
            runs.remove(r + 1);
        }
    }

    #[inline]
    fn collapse(runs: &[Run]) -> Option<usize> {
        let n = runs.len();
        if n >= 2
        {
            let right = runs[n - 1];
            let left = runs[n - 2];
            if right.power > left.power { Some(n - 2) } else { None }
        } else {
            None
        }
    }

    #[derive(Clone, Copy, Debug)]
    struct Run {
        start: usize,
        len: usize,
        power: u32
    }

    merge_cost
}

pub fn tim_sort<T, F>(v: &mut [T], mut is_less: F) -> usize
where
    F: FnMut(&T, &T) -> bool,
{
    let mut merge_cost = 0;
    const MAX_INSERTION: usize = 20;
    const MIN_RUN: usize = 10;

    if size_of::<T>() == 0 {
        return 0;
    }

    let len = v.len();

    if len <= MAX_INSERTION {
        if len >= 2 {
            for i in (0..len - 1).rev() {
                insert_head(&mut v[i..], &mut is_less);
            }
        }
        return 0;
    }

    let mut buf = Vec::with_capacity(len / 2);

    let mut runs = vec![];
    let mut end = len;
    while end > 0 {
        let mut start = end - 1;
        if start > 0 {
            start -= 1;
            unsafe {
                if is_less(v.get_unchecked(start + 1), v.get_unchecked(start)) {
                    while start > 0 && is_less(v.get_unchecked(start), v.get_unchecked(start - 1)) {
                        start -= 1;
                    }
                    v[start..end].reverse();
                } else {
                    while start > 0 && !is_less(v.get_unchecked(start), v.get_unchecked(start - 1))
                    {
                        start -= 1;
                    }
                }
            }
        }

        while start > 0 && end - start < MIN_RUN {
            start -= 1;
            insert_head(&mut v[start..end], &mut is_less);
        }

        runs.push(Run { start, len: end - start });
        end = start;

        while let Some(r) = collapse(&runs) {
            let left = runs[r + 1];
            let right = runs[r];
            unsafe {
                merge(
                    &mut v[left.start..right.start + right.len],
                    left.len,
                    buf.as_mut_ptr(),
                    &mut is_less,
                );
                merge_cost += left.len + right.len;
            }
            runs[r] = Run { start: left.start, len: left.len + right.len };
            runs.remove(r + 1);
        }
    }

    debug_assert!(runs.len() == 1 && runs[0].start == 0 && runs[0].len == len);

    #[inline]
    fn collapse(runs: &[Run]) -> Option<usize> {
        let n = runs.len();
        if n >= 2
            && (runs[n - 1].start == 0
                || runs[n - 2].len <= runs[n - 1].len
                || (n >= 3 && runs[n - 3].len <= runs[n - 2].len + runs[n - 1].len)
                || (n >= 4 && runs[n - 4].len <= runs[n - 3].len + runs[n - 2].len))
        {
            if n >= 3 && runs[n - 3].len < runs[n - 1].len { Some(n - 3) } else { Some(n - 2) }
        } else {
            None
        }
    }

    #[derive(Clone, Copy)]
    struct Run {
        start: usize,
        len: usize,
    }

    merge_cost
}

fn main() {
    let mut power_costs = Vec::with_capacity(100);
    let mut tim_costs = Vec::with_capacity(100);

    let mut is_less = |a: &i32, b: &i32| a < b;

    for _ in 0..100 {
        let mut sequence = generate_m_runs(100000000, 1000000);

        let power_cost = power_sort(&mut sequence.clone(), &mut is_less);
        let tim_cost = tim_sort(&mut sequence, &mut is_less);
        
        power_costs.push(power_cost);
        tim_costs.push(tim_cost);
    }

    println!("{:?}", power_costs);
    println!("{:?}", tim_costs);

    // Write the results to a file.
    let mut file = File::create("power_costs.txt").unwrap();
    for cost in power_costs {
        writeln!(file, "{}", cost).unwrap();
    }

    let mut file = File::create("tim_costs.txt").unwrap();
    for cost in tim_costs {
        writeln!(file, "{}", cost).unwrap();
    }
}