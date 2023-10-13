use std::{cmp::min, error::Error, io, result::Result as StdResult};

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
        let ans = soln(&a);
        println!("{ans}");
    }

    Ok(())
}

fn soln(a: &[u32]) -> usize {
    let n = a.len();
    let mut dp = vec![usize::MAX; n + 1];

    // BC
    dp[n] = 0;

    for i in (0..n).rev() {
        // two choices; kill or keep
        let kill = 1 + dp[i + 1];
        let idx = i + 1 + a[i] as usize;
        let keep = if idx <= n { dp[idx] } else { usize::MAX };
        dp[i] = min(kill, keep);
    }

    dp[0]
}
