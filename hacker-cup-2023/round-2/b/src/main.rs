use std::io;

use anyhow::Context;

fn main() -> anyhow::Result<()> {
    let mut lines = io::stdin().lines();
    let num_tests: usize = lines.next().context("num_tests")??.parse()?;

    for i in 1..=num_tests {
        let line = lines.next().context("line")??;
        let n = line.parse().unwrap();
        let a: Vec<_> = lines
            .next()
            .unwrap()
            .unwrap()
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        let b: Vec<_> = lines
            .next()
            .unwrap()
            .unwrap()
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        assert_eq!(a.len(), n);
        assert_eq!(b.len(), n);

        // quadruple each input, for good luck
        let mut aa = Vec::<u32>::with_capacity(n * 4);
        let mut bb = Vec::<u32>::with_capacity(n * 4);
        aa.extend(&a);
        aa.extend(&b);
        aa.extend(&a);
        aa.extend(&b);
        bb.extend(&b);
        bb.extend(&a);
        bb.extend(&b);
        bb.extend(&a);

        let ans = match soln(n, &aa, &bb) {
            Some(x) => x as isize,
            None => -1,
        };
        println!("Case #{i}: {ans}");
    }
    debug_assert!(lines.next().is_none());

    Ok(())
}

fn soln(n: usize, a: &[u32], b: &[u32]) -> Option<usize> {
    for c in candidates(n, a, b) {
        if is_match(n, a, b, c) {
            return Some(c);
        }
    }
    None
}

fn candidates(n: usize, a: &[u32], b: &[u32]) -> Vec<usize> {
    let mut out = vec![];

    let mut count = 0;
    for i in 0..3 * n {
        if a[i] < b[i] {
            count += 1;
        } else {
            if count >= n / 2 {
                out.push(i - n / 2);
            }
            count = 0;
        }
    }

    out
}

fn is_match(n: usize, a: &[u32], b: &[u32], c: usize) -> bool {
    for i in 0..n / 2 {
        if !(a[c + i] < b[c + i]) {
            return false;
        }
    }
    if n % 2 == 1 && !(a[c + n / 2] == b[c + n / 2]) {
        return false;
    }
    for i in ceil_div(n, 2)..n {
        if !(a[c + i] > b[c + i]) {
            return false;
        }
    }
    true
}

fn ceil_div(x: usize, y: usize) -> usize {
    assert_ne!(y, 0);
    let extra = if x % y != 0 { 1 } else { 0 };
    x / y + extra
}
