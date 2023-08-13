use std::{error::Error, io};

fn main() -> Result<(), Box<dyn Error>> {
    let mut lines = io::stdin().lines();
    let n = lines.next().ok_or("expected n")??.parse()?;

    let s = parse_coins(&lines.next().ok_or("expected s")??)?;
    let t = parse_coins(&lines.next().ok_or("expected t")??)?;
    assert_eq!(s.len(), n);
    assert_eq!(t.len(), n);

    let ij: Vec<_> = lines
        .next()
        .ok_or("expected i j")??
        .split_whitespace()
        .map(str::parse)
        .collect::<Result<_, _>>()?;
    let &[i, j] = ij.as_slice() else {
        Err("expected two nums: i j")?
    };
    assert!(i < j);
    assert!(j < n);

    if soln(&s, &t, i, j) {
        println!("YES");
    } else {
        println!("NO");
    }

    Ok(())
}

fn parse_coins(line: &str) -> Result<Vec<bool>, Box<dyn Error>> {
    line.chars()
        .map(|c| match c {
            'o' => Ok(false),
            'x' => Ok(true),
            _ => Err(format!("expected coin got {c:?}"))?,
        })
        .collect()
}

fn soln(s: &[bool], t: &[bool], i: usize, j: usize) -> bool {
    let n = s.len();
    assert!(i < j && j < n);
    assert_eq!(t.len(), n);

    // remove extra coins from s
    let extras = [s[i], s[j]];
    let s: Vec<_> = (0..n)
        .filter_map(|idx| {
            if idx == i || idx == j {
                None
            } else {
                Some(s[idx])
            }
        })
        .collect();
    debug_assert_eq!(s.len(), n - 2);

    // dp[j][i] will store the answer to the following subproblem:
    //
    // Is it possible to construct `t[i + j..]` by interleaving the sequences
    // `extras[j..]` and `s[i..]`?

    let mut dp = vec![vec![false; (n - 2) + 1]; 2 + 1];

    // base case: successfully used up all "extra" coins, and all of s
    dp[2][n - 2] = true;

    for j in (0..=2).rev() {
        for i in (0..=n - 2).rev() {
            if (j, i) == (2, n - 2) {
                continue;
            }

            // option A: use a coin from s
            let a = i < s.len() && s[i] == t[i + j] && dp[j][i + 1];

            // option B: use a coin from the extras
            let b = j < 2 && extras[j] == t[i + j] && dp[j + 1][i];

            dp[j][i] = a || b;
        }
    }

    dp[0][0]
}
