use std::{error::Error, io, result::Result as StdResult};

type Result<T> = StdResult<T, Box<dyn Error>>;

fn main() -> Result<()> {
    let mut lines = io::stdin().lines();
    let _num_tests: usize = lines.next().ok_or("no input")??.parse()?;

    while let Some(line) = lines.next() {
        let n = line?.parse()?;

        let b: Vec<_> = lines
            .next()
            .ok_or("expected b")??
            .split_whitespace()
            .map(str::parse)
            .collect::<StdResult<_, _>>()?;
        assert_eq!(b.len(), n);

        let ans = soln(&b);
        println!("{}", ans.len());
        for (i, x) in ans.into_iter().enumerate() {
            if i != 0 {
                print!(" ");
            }
            print!("{x}");
        }
        println!();
    }

    Ok(())
}

fn soln(b: &[u32]) -> Vec<u32> {
    let mut out = vec![];

    out.push(b[0]);

    for &x in &b[1..] {
        if x >= *out.last().unwrap() {
            out.push(x);
        } else {
            out.push(x);
            out.push(x);
        }
    }

    out
}
