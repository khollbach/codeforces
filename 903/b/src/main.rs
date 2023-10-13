use std::{error::Error, io, result::Result as StdResult};

type Result<T> = StdResult<T, Box<dyn Error>>;

fn main() -> Result<()> {
    let mut lines = io::stdin().lines();
    let _num_tests: usize = lines.next().ok_or("no input")??.parse()?;

    while let Some(line) = lines.next() {
        let mut nums: Vec<_> = line?
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        let ans = if soln(&mut nums) { "YES" } else { "NO" };
        println!("{ans}");
    }

    Ok(())
}

fn soln(nums: &mut [u32]) -> bool {
    assert_eq!(nums.len(), 3);
    nums.sort();
    assert_ne!(nums[0], 0);

    for i in 0..3 {
        if nums[i] % nums[0] != 0 {
            return false;
        }
    }

    let cuts_1 = nums[1] / nums[0] - 1;
    let cuts_2 = nums[2] / nums[0] - 1;
    (cuts_1 + cuts_2) <= 3
}
