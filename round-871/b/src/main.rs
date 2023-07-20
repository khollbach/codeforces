use std::{error::Error, io, result::Result as StdResult};

type Result<T> = StdResult<T, Box<dyn Error>>;

fn main() -> Result<()> {
    let mut lines = io::stdin().lines();
    let _num_tests = lines.next().ok_or("no input")??;

    while let Some(line) = lines.next() {
        let _n: usize = line?.parse()?;

        let line = lines.next().ok_or("expected a line of numbers")??;
        let nums: Vec<_> = line
            .split_whitespace()
            .map(|w| match w {
                "0" => Ok(0),
                "1" => Ok(1),
                _ => Err("not a binary digit"),
            })
            .collect::<StdResult<_, _>>()?;

        let ans = longest_blank(&nums);
        println!("{ans}");
    }

    Ok(())
}

fn longest_blank(nums: &[u8]) -> usize {
    let blanks = nums.split(|&x| x != 0);
    blanks.map(|slice| slice.len()).max().unwrap_or(0)
}
