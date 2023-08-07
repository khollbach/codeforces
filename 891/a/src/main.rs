use std::{error::Error, io, result::Result as StdResult};

type Result<T> = StdResult<T, Box<dyn Error>>;

fn main() -> Result<()> {
    let mut lines = io::stdin().lines();
    let _num_tests: usize = lines.next().ok_or("no input")??.parse()?;

    while let Some(line) = lines.next() {
        let n = line?.parse()?;

        let nums: Vec<_> = lines
            .next()
            .ok_or("expected line")??
            .split_whitespace()
            .map(str::parse)
            .collect::<StdResult<_, _>>()?;
        debug_assert_eq!(nums.len(), n);

        let ans = if soln(&nums) { "YES" } else { "NO" };
        println!("{ans}");
    }

    Ok(())
}

fn soln(nums: &[i32]) -> bool {
    let num_odds = nums.iter().filter(|&&x| x % 2 == 1).count();
    num_odds % 2 == 0
}
