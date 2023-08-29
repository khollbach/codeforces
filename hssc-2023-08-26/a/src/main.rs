use std::{error::Error, io, result::Result as StdResult};

type Result<T> = StdResult<T, Box<dyn Error>>;

fn main() -> Result<()> {
    let mut lines = io::stdin().lines();
    let _num_tests: usize = lines.next().ok_or("no input")??.parse()?;

    while let Some(line) = lines.next() {
        let xyn: Vec<_> = line?
            .split_whitespace()
            .map(str::parse)
            .collect::<StdResult<_, _>>()?;
        let &[x, y, n] = xyn.as_slice() else {
            Err("expected 3 nums")?
        };

        match soln(x, y, n as usize) {
            None => println!("-1"),
            Some(arr) => {
                for (i, x) in arr.into_iter().enumerate() {
                    if i != 0 {
                        print!(" ");
                    }
                    print!("{x}");
                }
                println!();
            }
        }
    }

    Ok(())
}

fn soln(x: i32, y: i32, n: usize) -> Option<Vec<i32>> {
    let mut arr = vec![-1; n];

    arr[n - 1] = y;

    let mut gap = 1;
    for i in (0..n - 1).rev() {
        arr[i] = arr[i + 1] - gap;
        gap += 1;
    }

    if arr[0] >= x {
        arr[0] = x;
        Some(arr)
    } else {
        None
    }
}
