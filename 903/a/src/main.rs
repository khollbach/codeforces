use std::{error::Error, io, result::Result as StdResult};

type Result<T> = StdResult<T, Box<dyn Error>>;

fn main() -> Result<()> {
    let mut lines = io::stdin().lines();
    let _num_tests: usize = lines.next().ok_or("no input")??.parse()?;

    while let Some(line) = lines.next() {
        let _nm = line?;
        let x = lines.next().unwrap().unwrap();
        let s = lines.next().unwrap().unwrap();

        match soln(x, &s) {
            Some(n) => println!("{n}"),
            None => println!("-1"),
        }
    }

    Ok(())
}

fn soln(mut x: String, s: &str) -> Option<usize> {
    for num_ops in 0.. {
        if x.contains(s) {
            return Some(num_ops);
        }

        x.push_str(&x.clone());

        if num_ops > 5 && x.len() > 5 * s.len() {
            break;
        }
    }

    None
}
