use std::{error::Error, io, result::Result as StdResult};

type Result<T> = StdResult<T, Box<dyn Error>>;

fn main() -> Result<()> {
    let mut lines = io::stdin().lines();
    let _num_tests: usize = lines.next().ok_or("no input")??.parse()?;

    while let Some(line) = lines.next() {
        let nums: Vec<_> = line?
            .split_whitespace()
            .map(str::parse)
            .collect::<StdResult<_, _>>()?;
        let nums = nums.try_into().map_err(|_| "expected 3 nums")?;

        let ans = if soln(nums) { "YES" } else { "NO" };
        println!("{ans}");
    }

    Ok(())
}

fn soln(mut nums: [u8; 3]) -> bool {
    nums.sort_unstable();
    let [_a, b, c] = nums;
    b + c >= 10
}
