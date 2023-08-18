use std::{error::Error, io, result::Result as StdResult};

type Result<T> = StdResult<T, Box<dyn Error>>;

fn main() -> Result<()> {
    let mut lines = io::stdin().lines();
    let _num_tests: usize = lines.next().ok_or("no input")??.parse()?;

    while let Some(line) = lines.next() {
        let ans = soln(&line?);

        match ans {
            None => println!("NO"),
            Some(s) => {
                println!("YES");
                println!("{s}");
            }
        }
    }

    Ok(())
}

fn soln(s: &str) -> Option<String> {
    let n = s.len();

    let s1 = "()".repeat(n);
    let s2 = "(".repeat(n) + &")".repeat(n);

    if !s1.contains(s) {
        Some(s1)
    } else if !s2.contains(s) {
        Some(s2)
    } else {
        None
    }
}
