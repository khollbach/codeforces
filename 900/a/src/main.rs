use std::{error::Error, io, result::Result as StdResult};

type Result<T> = StdResult<T, Box<dyn Error>>;

fn main() -> Result<()> {
    let mut lines = io::stdin().lines();
    let _num_tests: usize = lines.next().ok_or("no input")??.parse()?;

    while let Some(line) = lines.next() {
        let nk: Vec<_> = line?
            .split_whitespace()
            .map(str::parse)
            .collect::<StdResult<_, _>>()?;
        let &[n, k] = nk.as_slice() else {
            Err("nk")?
        };

        let a: Vec<_> = lines
            .next()
            .ok_or("a")??
            .split_whitespace()
            .map(str::parse)
            .collect::<StdResult<_, _>>()?;
        debug_assert_eq!(a.len(), n);

        let ans = if soln(&a, k as u32) { "YES" } else { "NO" };
        println!("{ans}");
    }

    Ok(())
}

fn soln(a: &[u32], x: u32) -> bool {
    a.contains(&x)
}
