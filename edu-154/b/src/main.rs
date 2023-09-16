use std::{error::Error, io, result::Result as StdResult};

type Result<T> = StdResult<T, Box<dyn Error>>;

fn main() -> Result<()> {
    let mut lines = io::stdin().lines();
    let _num_tests: usize = lines.next().ok_or("no input")??.parse()?;

    while let Some(line) = lines.next() {
        let a = line?;
        let b = lines.next().ok_or("expected b")??;

        let a = a.as_bytes();
        let b = b.as_bytes();
        let ans = if soln(a, b) { "YES" } else { "NO" };
        println!("{ans}");
    }

    Ok(())
}

fn soln(a: &[u8], b: &[u8]) -> bool {
    assert_eq!(a.len(), b.len());
    let n = a.len();
    assert!(n >= 2);

    for i in 0..n - 1 {
        if &a[i..i + 2] == b"01" && &b[i..i + 2] == b"01" {
            return true;
        }
    }
    false
}
