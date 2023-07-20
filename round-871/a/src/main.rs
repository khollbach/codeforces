use std::{error::Error, io, iter::zip, result::Result as StdResult};

type Result<T> = StdResult<T, Box<dyn Error>>;

fn main() -> Result<()> {
    let mut lines = io::stdin().lines();
    let _num_tests = lines.next().ok_or("no input")??;

    for line in lines {
        let ans = diff(&line?)?;
        println!("{ans}");
    }

    Ok(())
}

/// In how many positions does `s` differ from "codeforces"?
fn diff(s: &str) -> Result<usize> {
    if s.len() != 10 {
        Err("wrong length")?
    }
    if !s.chars().all(|c| c.is_ascii_lowercase()) {
        Err("non ascii-lowercase char")?
    }

    let d = zip(s.chars(), "codeforces".chars())
        .filter(|&(c1, c2)| c1 != c2)
        .count();
    Ok(d)
}
