use std::{error::Error, io, result::Result as StdResult, iter::zip};

type Result<T> = StdResult<T, Box<dyn Error>>;

fn main() -> Result<()> {
    let mut lines = io::stdin().lines();
    let _num_tests: usize = lines.next().ok_or("no input")??.parse()?;

    while let Some(line) = lines.next() {
        let n = line?.parse()?;
        let ans = soln(n);
        println!("{ans}");
    }

    Ok(())
}

fn soln(n: u64) -> u64 {
    std::cmp::max(_attempt_2(n), _attempt_3(n))
}

fn _attempt_3(n: u64) -> u64 {
    // idea: brute force the last ~10 elems in all permutations

    let mut best = 0;

    let mut nums: Vec<_> = (1..=n).collect();
    let n = n as usize;

    let base = n.saturating_sub(10);
    let (fixed_prefix, suffix) = nums.split_at_mut(base);
    let prefix_score = score(fixed_prefix);

    for p in perms(&suffix.to_vec()) {
        suffix.copy_from_slice(&p);

        let score = prefix_score + score(&suffix);
        if score > best {
            best = score;
        }
    }

    best
}

fn score(nums: &[u64]) -> u64 {
    let sum: u64 = zip(1.., nums).map(|(i, x)| i * x).sum();
    let max = zip(1.., nums).map(|(i, x)| i * x).max().unwrap_or(0);
    let score = sum - max;
    score
}

fn perms(nums: &[u64]) -> Vec<Vec<u64>> {
    if nums.is_empty() {
        return vec![vec![]];
    }

    let n_factorial = (1..=nums.len()).product();
    let mut out = Vec::with_capacity(n_factorial);

    for p in perms(&nums[1..]) {
        for i in 0..=p.len() {
            let mut new_p = Vec::with_capacity(nums.len());
            new_p.extend_from_slice(&p[..i]);
            new_p.push(nums[0]);
            new_p.extend_from_slice(&p[i..]);
            out.push(new_p);
        }
    }

    out
}

fn _attempt_2(n: u64) -> u64 {
    // idea: try swapping `n` with each other elem
    // find the swap that maximizes the sum of "squares" (minus largest)

    let mut nums: Vec<_> = (1..=n).collect();

    let mut best = 0;

    for idx in 0..n as usize {
        nums.swap(idx, n as usize - 1);

        let sum: u64 = zip(1.., &nums).map(|(i, x)| i * x).sum();
        let max = zip(1.., &nums).map(|(i, x)| i * x).max().unwrap();
        let score = sum - max;
        if score > best {
            best = score;
        }

        // undo the swap
        nums.swap(idx, n as usize - 1);
    }

    best
}

fn _attempt_1(n: u64) -> u64 {
    let mut sum = 0;

    // make the two largest rectangles the same; drop one
    sum += n * (n - 1);

    // the rest are just squares
    for i in 1..=n - 2 {
        sum += i.pow(2);
    }

    sum
}
