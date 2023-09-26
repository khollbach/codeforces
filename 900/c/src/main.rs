use std::{error::Error, io, result::Result as StdResult};

type Result<T> = StdResult<T, Box<dyn Error>>;

fn main() -> Result<()> {
    let mut lines = io::stdin().lines();
    let _num_tests: usize = lines.next().ok_or("no input")??.parse()?;

    while let Some(line) = lines.next() {
        let nkx: Vec<_> = line?
            .split_whitespace()
            .map(str::parse)
            .collect::<StdResult<_, _>>()?;
        let &[n, k, x] = nkx.as_slice() else {
            Err("nkx")?
        };
        let ans = if soln(n, k, x) { "YES" } else { "NO" };
        println!("{ans}");
    }

    Ok(())
}

// TODO: u64 overflowed, seemingly?
// but how? the numbers look small enough to me
//
// to-do: investigate...

fn soln(n: u128, k: u128, x: u128) -> bool {
    let min = sum_1_thru_n(k);
    let max = sum_i_thru_j(n - k + 1, n);

    // dbg!((n, k, x));
    // dbg!((min, x, max));
    min <= x && x <= max
}

fn sum_1_thru_n(n: u128) -> u128 {
    n * (n + 1) / 2
}

fn sum_i_thru_j(i: u128, j: u128) -> u128 {
    assert_ne!(i, 0);
    assert_ne!(j, 0);
    assert!(i <= j);

    sum_1_thru_n(j) - sum_1_thru_n(i - 1)
}
