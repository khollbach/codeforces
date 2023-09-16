use std::{error::Error, io, result::Result as StdResult, iter, collections::HashSet};

type Result<T> = StdResult<T, Box<dyn Error>>;

fn main() -> Result<()> {
    let mut lines = io::stdin().lines();
    let _num_tests: usize = lines.next().ok_or("no input")??.parse()?;

    while let Some(line) = lines.next() {
        let n = line?.parse()?;
        let nums: Vec<_> = lines
            .next()
            .ok_or("expected nums")??
            .split_whitespace()
            .map(str::parse)
            .collect::<StdResult<_, _>>()?;
        assert_eq!(nums.len(), n);

        let ans = soln(&nums);
        println!("{ans}");
    }

    Ok(())
}

fn soln(nums: &[u32]) -> usize {
    let mut ops = 0;

    let mut seen = HashSet::with_capacity(nums.len());
    for &x in nums {
        if x == 1 || seen.contains(&(x - 1)) {
            // good. no need to re-order this particular elem
        } else {
            // must do it at some point; might as well be now
            ops += 1;
        }
        seen.insert(x);
    }

    ops
}

// ---

pub fn _attempt_1(nums: &[u32]) -> usize {
    dbg!(nums);
    let nums = coalesce(nums);

    if nums.len() <= 1 {
        return 0;
    }

    // Choose an "optimal" pivot.
    let mid = nums.len() / 2;
    let mut sorted = nums.clone();
    sorted.sort_unstable();
    let pivot = sorted[mid];
    dbg!(pivot);

    let (left, right): (Vec<_>, _) = nums.iter().copied().partition(|&x| x < pivot);

    // "sort" both halves
    1 + _attempt_1(&left) + _attempt_1(&right)
}

// todo: hacky
fn coalesce(nums: &[u32]) -> Vec<u32> {
    if nums.is_empty() {
        return vec![];
    }

    let mut out = Vec::with_capacity(nums.len());
    let min = *nums.iter().min().unwrap() - 1;
    let max = *nums.iter().max().unwrap() + 1;

    for &x in iter::once(&min).chain(nums).chain([&max]) {
        if out.last().copied() == Some(x.saturating_sub(1)) {
            // skip x, since it's part of a continuous run.
        } else {
            out.push(x);
        }
    }

    out.remove(0);
    out.pop();

    out
}
