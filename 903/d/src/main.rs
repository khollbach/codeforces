use std::{collections::HashMap, error::Error, io, result::Result as StdResult};

type Result<T> = StdResult<T, Box<dyn Error>>;

fn main() -> Result<()> {
    let mut lines = io::stdin().lines();
    let _num_tests: usize = lines.next().ok_or("no input")??.parse()?;

    while let Some(line) = lines.next() {
        let n = line?.parse()?;
        let a: Vec<_> = lines
            .next()
            .unwrap()
            .unwrap()
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        assert_eq!(a.len(), n);

        let ans = if soln(&a) { "YES" } else { "NO" };
        println!("{ans}");
    }

    Ok(())
}

fn soln(a: &[u64]) -> bool {
    let n = a.len();
    let mut all_factors = HashMap::new();
    for &x in a {
        for (f, freq) in factor(x) {
            *all_factors.entry(f).or_default() += freq;
        }
    }
    all_factors.values().all(|freq| freq % n == 0)
}

fn factor(mut x: u64) -> HashMap<u64, usize> {
    assert_ne!(x, 0);
    let mut factors = HashMap::new();
    let mut i = 2;
    loop {
        while x % i == 0 {
            *factors.entry(i).or_default() += 1;
            x /= i;
        }
        i += 1;

        // todo: how to write this more cleanly?
        if i > x {
            break;
        } else if i.pow(2) > x {
            i = x;
        }
    }
    factors
}
