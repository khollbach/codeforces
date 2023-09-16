use std::{error::Error, io, result::Result as StdResult};

type Result<T> = StdResult<T, Box<dyn Error>>;

fn main() -> Result<()> {
    let mut lines = io::stdin().lines();
    let _num_tests: usize = lines.next().ok_or("no input")??.parse()?;

    while let Some(line) = lines.next() {
        let lr: Vec<_> = line?.split_whitespace().map(str::parse).collect::<StdResult<_, _>>()?;
        let &[l, r] = lr.as_slice() else {
            Err("expected 2 nums")?
        };
        match soln(l, r) {
            Some((x, y)) => println!("{x} {y}"),
            None => println!("-1"),
        }
    }

    Ok(())
}

fn smallest_factor(x: u32) -> u32 {
    let mut i = 2;
    while i * i <= x {
        if x % i == 0 {
            return i;
        }
        i += 1;
    }
    x
}

fn soln(l: u32, r: u32) -> Option<(u32, u32)> {
    for x in l..=r {
        let f = smallest_factor(x);
        if f == x {
            // no non-trivial factors
            continue;
        }

        return Some((f, f * (x / f - 1)));
    }

    None
}
