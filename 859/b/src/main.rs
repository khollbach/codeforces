use std::{error::Error, io, result::Result as StdResult};

type Result<T> = StdResult<T, Box<dyn Error>>;

fn main() -> Result<()> {
    let mut lines = io::stdin().lines();
    let _num_tests = lines.next().ok_or("no input")??;

    while let Some(line) = lines.next() {
        let _nums_len: usize = line?.parse()?;

        let Some(line) = lines.next() else {
            Err("expected line of nums")?
        };
        let nums: Vec<_> = line?
            .split_whitespace()
            .map(str::parse)
            .collect::<StdResult<_, _>>()?;

        let ans = if evens_gt_odds(&nums) { "YES" } else { "NO" };
        println!("{ans}");
    }

    Ok(())
}

fn evens_gt_odds(nums: &[u32]) -> bool {
    let even_sum: u32 = nums.iter().filter(|&&x| x % 2 == 0).sum();
    let odd_sum: u32 = nums.iter().filter(|&&x| x % 2 != 0).sum();
    even_sum > odd_sum
}
