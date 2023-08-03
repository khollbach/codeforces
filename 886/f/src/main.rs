use std::{collections::HashMap, error::Error, io, result::Result as StdResult};

type Result<T> = StdResult<T, Box<dyn Error>>;

fn main() -> Result<()> {
    let mut lines = io::stdin().lines();
    let _num_tests: usize = lines.next().ok_or("no input")??.parse()?;

    while let Some(line) = lines.next() {
        let n: usize = line?.parse()?;
        let nums: Vec<_> = lines
            .next()
            .ok_or("expected nums")??
            .split_whitespace()
            .map(str::parse)
            .collect::<StdResult<_, _>>()?;
        debug_assert_eq!(nums.len(), n);

        let ans = soln(&nums).expect("nums must be non-empty");
        println!("{ans}");
    }

    Ok(())
}

/// O(n * sqrt(n))
fn soln(nums: &[u32]) -> Option<usize> {
    let n = nums.len();

    let mut freqs = HashMap::with_capacity(n);
    for a in nums {
        *freqs.entry(a).or_default() += 1;
    }

    (1..=n as u32)
        .map(|x| divisors(x).filter_map(|d| freqs.get(&d)).sum())
        .max()
}

/// O(sqrt(n)) -- cool!
fn divisors(n: u32) -> impl Iterator<Item = u32> {
    assert_ne!(n, 0);
    assert_ne!(n, u32::MAX);

    (1u32..)
        .take_while(move |i| i.saturating_pow(2) <= n)
        .flat_map(move |i| {
            if i * i == n {
                vec![i]
            } else if n % i == 0 {
                vec![i, n / i]
            } else {
                vec![]
            }
        })
}
