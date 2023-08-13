use std::{error::Error, io, iter::zip, result::Result as StdResult};

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

fn _main() {
    for i in 0..=250 {
        dbg!(soln(i));
    }
}

fn soln(n: u64) -> u64 {
    let guess = generalized(n);
    // let actual = brute_force(n);
    // assert_eq!(guess, actual);
    guess
}

// Omega(n * n!)
fn brute_force(n: u64) -> u64 {
    let nums: Vec<_> = (1..=n).collect();
    let (score, p) = perms(&nums)
        .into_iter()
        .map(|p| (score(&p), p))
        .max_by_key(|&(score, _)| score)
        .unwrap();

    // let areas: Vec<_> = zip(1.., &p).map(|(i, x)| i * x).collect();
    // dbg!(&p, areas, score);
    score
}

// does this work? If so, I have *no* idea why...
fn generalized(n: u64) -> u64 {
    let mut nums: Vec<_> = (1..=n).collect();
    (0..=n as usize)
        .map(|i| {
            nums[i..].reverse();
            let score = score(&nums);
            nums[i..].reverse();
            score
        })
        .max()
        .unwrap_or(0)
}

fn _naive(n: u64) -> u64 {
    if n <= 1 {
        return 0;
    }

    let mut perm = Vec::with_capacity(n as usize);
    let mut areas = Vec::with_capacity(n as usize);
    let mut sum = 0;

    // most are just squares
    for i in 1..=n - 2 {
        let area = i.pow(2);
        sum += area;

        perm.push(i);
        areas.push(area);
    }

    // make the two largest rectangles the same; drop one
    let area = n * (n - 1);
    sum += area;

    perm.push(n);
    perm.push(n - 1);
    areas.push(area);
    areas.push(area);

    // dbg!(perm, areas, sum);
    sum
}

// ---

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

// O((n!)^2) maybe?
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
