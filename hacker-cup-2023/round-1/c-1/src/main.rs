use std::io;

use anyhow::Context;

fn main() -> anyhow::Result<()> {
    let mut lines = io::stdin().lines();
    let num_tests: usize = lines.next().context("num_tests")??.parse()?;

    for i in 1..=num_tests {
        let line = lines.next().context("n")??;
        let n = line.parse()?;
        let s = lines.next().context("s")??;
        let s = bitstring_to_vec(&s);
        assert_eq!(s.len(), n);

        let q = lines.next().context("q")??.parse()?;
        let queries: Vec<usize> = lines
            .by_ref()
            .take(q)
            .map(|line| line.unwrap().parse().unwrap())
            .collect();
        assert_eq!(queries.len(), q);

        let ans = soln(s, &queries);
        println!("Case #{i}: {ans}");
    }
    debug_assert!(lines.next().is_none());

    Ok(())
}

fn bitstring_to_vec(s: &str) -> Vec<bool> {
    let mut bits = vec![];
    for c in s.chars() {
        let b = match c {
            '0' => false,
            '1' => true,
            _ => panic!("not a bit: {c:?}"),
        };
        bits.push(b);
    }
    bits
}

fn soln(mut s: Vec<bool>, qs: &[usize]) -> usize {
    let n = s.len();
    s.insert(0, false); // shift over, so we can use 1-based indexing.

    // batch queries
    let mut queries = vec![false; n + 1];
    for &i in qs {
        queries[i] ^= true;
    }

    // apply queries to s
    for i in 1..=n {
        if queries[i] {
            let mut idx = i;
            while idx <= n {
                s[idx] ^= true;
                idx += i;
            }
        }
    }

    // do the greedy thing to make `s` all 0s.
    let mut num_ops = 0;
    for i in 1..=n {
        if s[i] {
            let mut idx = i;
            while idx <= n {
                s[idx] ^= true;
                idx += i;
            }
            num_ops += 1;
        }
    }
    num_ops
}
