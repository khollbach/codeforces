use std::{collections::BTreeMap, error::Error, io, result::Result as StdResult};

type Result<T> = StdResult<T, Box<dyn Error>>;

fn main() -> Result<()> {
    let mut lines = io::stdin().lines();
    let _num_tests: usize = lines.next().ok_or("no input")??.parse()?;

    while let Some(line) = lines.next() {
        let n = line?.parse()?;
        let nums: Vec<_> = lines
            .next()
            .ok_or("expected line")??
            .split_whitespace()
            .map(str::parse)
            .collect::<StdResult<_, _>>()?;

        let ans = soln(n, &nums);
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

/// Upper limit, inclusive.
const INFTY: i32 = 10i32.pow(9);

fn soln(n: usize, nums: &[i32]) -> Vec<i32> {
    assert!(n >= 2);
    assert_eq!(nums.len(), n * (n - 1) / 2);
    assert!(nums.iter().all(|&x| x.abs() <= INFTY));

    let mut freqs = BTreeMap::<i32, usize>::new();
    for &x in nums {
        *freqs.entry(x).or_default() += 1;
    }

    let mut out = Vec::with_capacity(n);

    for i in (1..n).rev() {
        let mut entry = freqs.first_entry().unwrap();
        let x = *entry.key();

        // Consume i copies of x.
        let f = entry.get_mut();
        *f = f.checked_sub(i).unwrap();
        if *f == 0 {
            entry.remove();
        }

        out.push(x);
    }

    out.push(INFTY);

    out
}
