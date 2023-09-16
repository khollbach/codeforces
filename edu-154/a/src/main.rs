use std::{error::Error, io, result::Result as StdResult};

type Result<T> = StdResult<T, Box<dyn Error>>;

fn main() -> Result<()> {
    let mut lines = io::stdin().lines();
    let _num_tests: usize = lines.next().ok_or("no input")??.parse()?;

    while let Some(line) = lines.next() {
        let s = line?;
        let ans = soln(&s);
        println!("{ans}");
    }

    Ok(())
}

fn soln(s: &str) -> u32 {
    for c in s.chars() {
        if c == '1' {
            return 13;
        }
        if c == '3' {
            return 31;
        }
    }
    panic!("must contain 1 and 3");
}
