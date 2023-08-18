use std::{cmp::min, error::Error, io, result::Result as StdResult};

type Result<T> = StdResult<T, Box<dyn Error>>;

fn main() -> Result<()> {
    let mut lines = io::stdin().lines();
    let _num_tests: usize = lines.next().ok_or("no input")??.parse()?;

    while let Some(line) = lines.next() {
        let nums: Vec<_> = line?
            .split_whitespace()
            .map(str::parse)
            .collect::<StdResult<_, _>>()?;
        let &[m, k, a_1, a_k] = nums.as_slice() else {
            Err("expected 4 nums")?
        };

        let ans = soln(m, k, a_1, a_k);
        println!("{ans}");
    }

    Ok(())
}

fn num_fancy(m: u32, k: u32) -> u32 {
    assert_ne!(k, 0);
    m / k + m % k
}

fn soln(m: u32, k: u32, a_1: u32, a_k: u32) -> u32 {
    // immediately spend big coins
    let big = min(a_k, m / k);
    let m = m - big * k;

    // two choices:
    // * spend to minimize m / k
    // * spend to minimize m % k

    let m_1 = {
        let small = min(a_1, m);
        m - small
    };

    let m_2 = {
        let small = min(a_1, m % k);
        let m = m - small;
        let a_1 = a_1 - small;

        let small_groups = min(a_1 / k, m / k);
        m - small_groups * k
    };

    min(num_fancy(m_1, k), num_fancy(m_2, k))
}
