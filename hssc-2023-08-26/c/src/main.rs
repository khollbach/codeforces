use std::{error::Error, io, result::Result as StdResult};

type Result<T> = StdResult<T, Box<dyn Error>>;

fn main() -> Result<()> {
    let mut lines = io::stdin().lines();
    let _num_tests: usize = lines.next().ok_or("no input")??.parse()?;

    while let Some(line) = lines.next() {
        let x = line?.parse()?;
        let ans = soln(x);
        println!("{}", ans.len());
        for (i, a) in ans.into_iter().enumerate() {
            if i != 0 {
                print!(" ");
            }
            print!("{a}");
        }
        println!();
    }

    Ok(())
}

fn soln(mut x: u32) -> Vec<u32> {
    let diffs = diffs(x);

    let mut out = Vec::with_capacity(diffs.len() + 1);

    out.push(x);
    for d in diffs {
        x -= d;
        out.push(x);
    }

    debug_assert_eq!(*out.last().unwrap(), 1);
    out
}

fn diffs(mut x: u32) -> Vec<u32> {
    assert_ne!(x, 0);
    let mut diffs = vec![];

    for i in 0.. {
        debug_assert!(i < 32);

        let bit = 1 << i;
        if x & bit == x {
            // Stop before zero-ing the last one.
            break;
        }

        // Zero-out ones.
        if x & bit != 0 {
            diffs.push(bit);
            x ^= bit;
        }
    }

    debug_assert!(x.is_power_of_two());

    // Insert all powers of 2 less than x, in decreasing order.
    loop {
        x >>= 1;
        if x == 0 {
            break;
        }
        diffs.push(x);
    }

    diffs
}
