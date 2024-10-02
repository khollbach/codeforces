use std::{cmp::max, error::Error, io};

type Result<T> = std::result::Result<T, Box<dyn Error>>;

fn main() -> Result<()> {
    let mut lines = io::stdin().lines();
    let t: usize = lines.next().ok_or("t")??.parse()?;

    for _ in 0..t {
        let n: usize = lines.next().ok_or("n")??.parse()?;
        let mut a: Vec<i64> = lines
            .next()
            .ok_or("nums")??
            .split_whitespace()
            .map(str::parse)
            .collect::<std::result::Result<_, _>>()?;
        assert_eq!(a.len(), n);

        if let Some(ans) = min_cost_to_summon(&mut a) {
            println!("{ans}");
        } else {
            println!("-1");
        }
    }

    assert!(lines.next().is_none());
    Ok(())
}

fn min_cost_to_summon(a: &mut [i64]) -> Option<i64> {
    a.sort_unstable();

    let n = a.len();
    if n <= 2 {
        return None;
    }

    let sum: i64 = a.iter().sum();
    let x = max(0, 2 * (n as i64) * a[n / 2] - sum + 1); // BUG: dropped the 2*
    Some(x)
}
