use std::{error::Error, io, result::Result as StdResult};

type Result<T> = StdResult<T, Box<dyn Error>>;

fn main() -> Result<()> {
    let mut lines = io::stdin().lines();
    let _num_tests: usize = lines.next().ok_or("no input")??.parse()?;

    while let Some(line) = lines.next() {
        let n = line?.parse()?;

        let nums: Vec<_> = lines
            .next()
            .ok_or("expected nums")??
            .split_whitespace()
            .map(str::parse)
            .collect::<StdResult<_, _>>()?;
        assert_eq!(nums.len(), n);

        let ans = if soln(&nums) { "YES" } else { "NO" };
        println!("{ans}");
    }

    Ok(())
}

fn soln(heights: &[u32]) -> bool {
    let n = heights.len() as u32;

    if heights[0] > n {
        return false;
    }

    let mut widths = vec![];

    let mut prev = 0;
    for (i, &h) in heights.iter().rev().enumerate() {
        if h > prev {
            let gap = h - prev;
            for _ in 0..gap {
                if widths.len() >= heights.len() {
                    return false;
                }
                widths.push(n - i as u32);
            }
        }
        prev = h;
    }

    widths == heights
}
