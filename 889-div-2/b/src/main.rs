use std::{error::Error, io, result::Result as StdResult};

type Result<T> = StdResult<T, Box<dyn Error>>;

fn main() -> Result<()> {
    let mut lines = io::stdin().lines();
    let _num_tests: usize = lines.next().ok_or("no input")??.parse()?;

    while let Some(line) = lines.next() {
        let n = line?.parse()?;
        let ans = soln(n);
        println!("{ans}");
    }

    Ok(())
}

/// Num consecutive divisors.
fn soln(n: u64) -> usize {
    (1..=n).take_while(|&d| n % d == 0).count()
}
