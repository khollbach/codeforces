use std::{error::Error, io, result::Result as StdResult};

type Result<T> = StdResult<T, Box<dyn Error>>;

fn main() -> Result<()> {
    let mut lines = io::stdin().lines();
    let _num_tests: usize = lines.next().ok_or("no input")??.parse()?;

    while let Some(line) = lines.next() {
        let n = line?.parse()?;
        let ans = soln(n);
        for x in ans {
            print!("{x} ");
        }
        println!();
    }

    Ok(())
}

fn soln(n: usize) -> Vec<usize> {
    assert!(n >= 3);

    let mut out = Vec::with_capacity(n);

    let mut x = 100;
    for i in 0..n {
        out.push(x);
        x += 1;

        if i >= 2 {
            let left = 3 * out[i];
            let right = out[i - 2] + out[i - 1];
            debug_assert_ne!(left % right, 0, "{i}: {:?}", &out[i - 2..=i]);
        }
    }

    out
}
