use std::{collections::HashMap, error::Error, io, result::Result as StdResult};

type Result<T> = StdResult<T, Box<dyn Error>>;

fn main() -> Result<()> {
    let mut lines = io::stdin().lines();
    let _num_tests: usize = lines.next().ok_or("no input")??.parse()?;

    while let Some(line) = lines.next() {
        let n = line?.parse()?;

        let nums: Vec<_> = lines
            .next()
            .ok_or("expected line of nums")??
            .split_whitespace()
            .map(str::parse)
            .collect::<StdResult<_, _>>()?;
        debug_assert_eq!(nums.len(), n);

        let ans = if soln(nums) { "YES" } else { "NO" };
        println!("{ans}");
    }

    Ok(())
}

fn soln(mut nums: Vec<u32>) -> bool {
    match nums.len() {
        0 => return true,
        1 => return false,
        _ => (),
    }

    debug_assert!(nums.iter().all(|&x| x != 0));
    for x in &mut nums {
        *x -= 1;
    }

    let num_zeros = nums.iter().filter(|&&x| x == 0).count();
    let nonzero_total: u64 = nums.iter().map(|&x| x as u64).sum();

    // wlog, remove all points from people that started with any.
    // As long as we can give *something* to every zero, we win.
    nonzero_total as usize >= num_zeros
}

fn _wrong_soln(nums: &[u32]) -> bool {
    let n = nums.len();

    let mut freqs = HashMap::with_capacity(n);
    for &x in nums {
        *freqs.entry(x).or_default() += 1;
    }

    let max_freq = freqs.into_iter().map(|(_x, f)| f).max().unwrap_or(0);

    max_freq * 2 <= n
}
