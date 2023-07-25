use std::{error::Error, io, iter::zip, result::Result as StdResult};

type Result<T> = StdResult<T, Box<dyn Error>>;

fn main() -> Result<()> {
    let mut lines = io::stdin().lines();
    let _num_tests: usize = lines.next().ok_or("no input")??.parse()?;

    while let Some(line) = lines.next() {
        let n = line?.parse()?;

        let nums: Vec<_> = lines
            .next()
            .ok_or("expected lines of nums")??
            .split_whitespace()
            .map(str::parse)
            .collect::<StdResult<_, _>>()?;
        debug_assert_eq!(nums.len(), n);

        let ans = if parity_sort(&nums) { "YES" } else { "NO" };
        println!("{ans}");
    }

    Ok(())
}

fn parity_sort(nums: &[u32]) -> bool {
    let n = nums.len();

    let even_idxs = nums
        .iter()
        .enumerate()
        .filter(|&(_, &x)| x % 2 == 0)
        .map(|(i, _)| i);
    let odd_idxs = nums
        .iter()
        .enumerate()
        .filter(|&(_, &x)| x % 2 != 0)
        .map(|(i, _)| i);

    let mut sorted_evens: Vec<_> = nums.iter().copied().filter(|&x| x % 2 == 0).collect();
    let mut sorted_odds: Vec<_> = nums.iter().copied().filter(|&x| x % 2 != 0).collect();
    sorted_evens.sort_unstable();
    sorted_odds.sort_unstable();

    let mut maybe_sorted = vec![u32::MAX; n];
    let mut debug_n = 0;
    for (i, x) in zip(even_idxs, sorted_evens) {
        maybe_sorted[i] = x;
        debug_n += 1;
    }
    for (i, x) in zip(odd_idxs, sorted_odds) {
        maybe_sorted[i] = x;
        debug_n += 1;
    }
    debug_assert_eq!(debug_n, n);
    debug_assert!(maybe_sorted.iter().all(|&x| x != u32::MAX));

    // Sorted?
    // ACK. I wrote this as strictly-less-than the first time around :/
    maybe_sorted.windows(2).all(|pair| pair[0] <= pair[1])
}
