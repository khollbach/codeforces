use std::{error::Error, io, result::Result as StdResult};

type Result<T> = StdResult<T, Box<dyn Error>>;

fn main() -> Result<()> {
    let mut lines = io::stdin().lines();
    let _num_tests: usize = lines.next().ok_or("no input")??.parse()?;

    while let Some(line) = lines.next() {
        let n: usize = line?.parse()?;

        let mut nums: Vec<_> = lines
            .next()
            .ok_or("expected line of nums")??
            .split_whitespace()
            .map(str::parse)
            .collect::<StdResult<_, _>>()?;
        debug_assert_eq!(nums.len(), n);

        let ans = sort_general_case(&mut nums);

        let k = ans.len();
        debug_assert_eq!(k, n * 2);
        println!("{k}");

        debug_assert!(nums.windows(2).all(|pair| pair[0] <= pair[1]));

        for (i, j) in ans {
            // 1-indexed.
            println!("{} {}", i + 1, j + 1);
        }
    }

    Ok(())
}

fn sort_general_case(nums: &mut [i64]) -> Vec<(usize, usize)> {
    if nums.is_empty() {
        return vec![];
    }

    let n = nums.len();
    let mut ops = Vec::with_capacity(n * 2);

    let (max_idx, &max_val) = nums.iter().enumerate().max_by_key(|&(_, &x)| x).unwrap();
    let (min_idx, &min_val) = nums.iter().enumerate().min_by_key(|&(_, &x)| x).unwrap();

    if max_val >= -min_val {
        // Ensure all nums >= 0.
        for i in 0..n {
            op(nums, &mut ops, i, max_idx);
        }
        sort_all_positive(nums, &mut ops);
    } else {
        // Ensure all nums <= 0.
        for i in 0..n {
            op(nums, &mut ops, i, min_idx);
        }
        sort_all_negative(nums, &mut ops);
    }

    ops
}

fn op(nums: &mut [i64], ops: &mut Vec<(usize, usize)>, i: usize, j: usize) {
    nums[i] += nums[j];
    ops.push((i, j));
}

/// Special case: all nums >= 0.
fn sort_all_positive(nums: &mut [i64], ops: &mut Vec<(usize, usize)>) {
    assert!(nums.iter().all(|&x| x >= 0));

    // Ensure the first element is the largest.
    let Some((max_idx, _)) = nums.iter().enumerate().max_by_key(|&(_, &x)| x) else {
        return;
    };
    op(nums, ops, 0, max_idx);

    // Invariant: the current element is the largest.
    for i in 1..nums.len() {
        op(nums, ops, i, i - 1);
    }
}

/// Special case: all nums <= 0.
fn sort_all_negative(nums: &mut [i64], ops: &mut Vec<(usize, usize)>) {
    assert!(nums.iter().all(|&x| x <= 0));
    let n = nums.len();

    // Ensure the last element is the smallest.
    let Some((min_idx, _)) = nums.iter().enumerate().min_by_key(|&(_, &x)| x) else {
        return;
    };
    op(nums, ops, n - 1, min_idx);

    // Invariant: the current element is the smallest.
    for i in (0..n - 1).rev() {
        op(nums, ops, i, i + 1);
    }
}
