use std::{error::Error, io};

fn main() -> Result<(), Box<dyn Error>> {
    let mut lines = io::stdin().lines();
    let _num_tests = lines.next().ok_or("no input")??;

    for line in lines {
        let nums: Vec<_> = line?
            .split_whitespace()
            .map(str::parse)
            .collect::<Result<_, _>>()?;
        let &[a, b, c] = nums.as_slice() else {
            Err("expected three words")?
        };

        let ans = if sum(a, b, c) { "YES" } else { "NO" };
        println!("{ans}");
    }

    Ok(())
}

fn sum(a: u32, b: u32, c: u32) -> bool {
    let mut nums = vec![a, b, c];
    nums.sort_unstable();
    let &[a, b, c] = nums.as_slice() else {
        unreachable!()
    };

    a + b == c
}
