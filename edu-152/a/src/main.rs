use std::{error::Error, io, result::Result as StdResult, cmp::min};

type Result<T> = StdResult<T, Box<dyn Error>>;

fn main() -> Result<()> {
    let mut lines = io::stdin().lines();
    let _num_tests: usize = lines.next().ok_or("no input")??.parse()?;

    while let Some(line) = lines.next() {
        let bch: Vec<_> = line?.split_whitespace().map(str::parse).collect::<StdResult<_, _, >>()?;
        let &[b, c, h] = bch.as_slice() else {
            Err("expected 3 nums: b, c, h")?
        };

        let ans = soln(b, c, h);
        println!("{ans}");
    }

    Ok(())
}

fn soln(b: u32, c: u32, h: u32) -> u32 {
    assert!(b >= 2);
    assert_ne!(c, 0);
    assert_ne!(h, 0);

    min(c + h, b - 1) * 2 + 1
}
