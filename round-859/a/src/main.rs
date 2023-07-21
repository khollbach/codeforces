use std::{error::Error, io, result::Result as StdResult};

type Result<T> = StdResult<T, Box<dyn Error>>;

fn main() -> Result<()> {
    let mut lines = io::stdin().lines();
    let _num_tests = lines.next().ok_or("no input")??;

    for line in lines {
        let nums: Vec<_> = line?
            .split_whitespace()
            .map(str::parse)
            .collect::<StdResult<_, _>>()?;
        let &[a, b, c] = nums.as_slice() else {
            Err("expected 3 nums")?
        };

        let ans = if is_plus(a, b, c) { '+' } else { '-' };
        println!("{ans}");
    }

    Ok(())
}

fn is_plus(a: i32, b: i32, c: i32) -> bool {
    debug_assert!((a + b == c) ^ (a - b == c));
    a + b == c
}
