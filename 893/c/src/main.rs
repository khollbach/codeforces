use std::{collections::HashSet, error::Error, io, result::Result as StdResult};

type Result<T> = StdResult<T, Box<dyn Error>>;

fn main() -> Result<()> {
    let mut lines = io::stdin().lines();
    let _num_tests: usize = lines.next().ok_or("no input")??.parse()?;

    while let Some(line) = lines.next() {
        let n = line?.parse()?;

        let ans = soln(n);
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

fn soln(n: u32) -> Vec<u32> {
    let mut out = Vec::with_capacity(n as usize);
    let mut seen = HashSet::with_capacity(n as usize);

    for mut x in (1..=n).rev() {
        while !seen.contains(&x) {
            seen.insert(x);
            out.push(x);

            if x % 2 != 0 {
                break;
            }
            x /= 2;
        }
    }

    // for pair in out.windows(2) {
    //     print!("{} ", gcd(pair[0], pair[1]));
    // }
    // print!("{} ***", gcd(*out.last().unwrap(), out[0]));
    // println!();

    out
}

fn _gcd(mut a: u32, mut b: u32) -> u32 {
    assert_ne!(a, 0);
    assert_ne!(b, 0);

    let sort = |a, b| if a <= b { (a, b) } else { (b, a) };

    // wlog, a <= b
    (a, b) = sort(a, b);

    while a != 0 {
        (a, b) = sort(b - a, a);
    }

    b
}
