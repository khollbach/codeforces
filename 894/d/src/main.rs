use std::{
    cmp::Ordering::{Equal, Greater, Less},
    error::Error,
    io,
    result::Result as StdResult,
};

type Result<T> = StdResult<T, Box<dyn Error>>;

fn main() -> Result<()> {
    let mut lines = io::stdin().lines();
    let _num_tests: usize = lines.next().ok_or("no input")??.parse()?;

    while let Some(line) = lines.next() {
        let target = line?.parse()?;
        let ans = soln(target);
        println!("{ans}");
    }

    Ok(())
}

/// Find the minimum n such that `(n choose 2) >= target`.
fn soln(target: u128) -> u128 {
    let mut lo = 1;
    let mut hi = u64::MAX as u128; // exclusive

    while lo < hi {
        let mid = (lo + hi) / 2;
        let value = mid * (mid - 1) / 2;
        match target.cmp(&value) {
            Equal => return mid,
            Less => hi = mid,
            Greater => lo = mid + 1,
        }
    }
    debug_assert_eq!(lo, hi);

    // We failed to find an exact `n`, but we're now looking at an empty slice
    // that is between the "greatest lower bound" and the "least upper bound".
    //
    // So we can return the LUB, aka "ceiling" of the fractional answer.

    lo
}

// Hmm, now that I understand what the problem is asking, this seems pretty hard
// and mathy. I'll have to think about it some more.
