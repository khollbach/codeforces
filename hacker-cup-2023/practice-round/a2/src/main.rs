use std::{io, cmp::min};

use anyhow::Context;
use itertools::Itertools;

fn main() -> anyhow::Result<()> {
    let mut lines = io::stdin().lines();
    let num_tests: usize = lines.next().context("num_tests")??.parse()?;

    for i in 1..=num_tests {
        let line = lines.next().context("line")??;
        let (a, b, c) = line
            .split_whitespace()
            .map(|s| s.parse().expect("parse"))
            .collect_tuple()
            .context("abc")?;

        let ans = soln(a, b, c);
        println!("Case #{i}: {ans}");
    }
    debug_assert!(lines.next().is_none());

    Ok(())
}

fn soln(s: u64, d: u64, cash: u64) -> u64 {
    if d >= 2 * s {
        ignore_doubles(s, cash)
    } else if d <= s {
        ignore_singles(d, cash)
    } else {
        assert!(s < d && d < 2 * s);

        // ok, doubles are a good deal, so we mostly want
        // to use as many as possible.

        // but there's gonna be some edge cases where we want
        // to buy singles -- so let's just brute force over
        // some small values of `num_singles`

        (0..5)
            .map(|num_singles| {
                let cost = num_singles * s;
                if cost > cash {
                    return 0;
                }

                // buy that many singles
                let mut num_buns = num_singles * 2;
                let mut num_patties = num_singles;
                let cash = cash - cost;

                // buy as many doubles as possible
                let num_doubles = cash / d;
                num_buns += num_doubles * 2;
                num_patties += num_doubles * 2;

                min(num_buns.saturating_sub(1), num_patties)
            })
            .max()
            .unwrap()
    }
}

fn ignore_doubles(s: u64, cash: u64) -> u64 {
    let num_singles = cash / s;
    let num_buns = num_singles * 2;
    let num_patties = num_singles;

    min(num_buns.saturating_sub(1), num_patties)
}

fn ignore_singles(d: u64, cash: u64) -> u64 {
    let num_doubles = cash / d;
    let num_buns = num_doubles * 2;
    let num_patties = num_doubles * 2;

    min(num_buns.saturating_sub(1), num_patties)
}
