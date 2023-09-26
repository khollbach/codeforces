use std::io;

use anyhow::Context;
use itertools::Itertools;

fn main() -> anyhow::Result<()> {
    let mut lines = io::stdin().lines();
    let num_tests: usize = lines.next().context("num_tests")??.parse()?;

    for i in 1..=num_tests {
        let line = lines.next().context("line")??;
        let (r, c, a, b) = line
            .split_whitespace()
            .map(|s| s.parse().expect("parse"))
            .collect_tuple()
            .context("collect_tuple")?;

        let ans = if soln(r, c, a, b) { "YES" } else { "NO" };
        println!("Case #{i}: {ans}");
    }
    debug_assert!(lines.next().is_none());

    Ok(())
}

/// Does Alice win?
fn soln(r: u32, c: u32, _a: u32, _b: u32) -> bool {
    // Longer side wins. Alice loses ties.
    r > c
}
