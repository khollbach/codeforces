use std::io;

use anyhow::Context;
use itertools::Itertools;

fn main() -> anyhow::Result<()> {
    let mut lines = io::stdin().lines();
    let num_tests: usize = lines.next().context("num_tests")??.parse()?;

    for i in 1..=num_tests {
        let line = lines.next().context("line")??;
        let (s, d, k) = line
            .split_whitespace()
            .map(|s| s.parse().expect("parse"))
            .collect_tuple()
            .context("collect_tuple")?;

        let ans = if soln(s, d, k) { "YES" } else { "NO" };
        println!("Case #{i}: {ans}");
    }
    debug_assert!(lines.next().is_none());

    Ok(())
}

fn soln(s: u32, d: u32, k: u32) -> bool {
    let num_buns = s * 2 + d * 2;
    let num_patties = s + d * 2;

    let patties_needed = k;
    let buns_needed = k + 1;

    num_buns >= buns_needed && num_patties >= patties_needed
}
