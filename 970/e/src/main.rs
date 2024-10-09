/*

This clearly isn't the approach they want.
We timed out at 2 seconds, and then succeeded at 1.9 seconds.
But hey, I'll take it.

TODO: look up their approach.

*/

use std::{cmp::min, collections::HashMap, io};

fn main() {
    let mut lines = io::stdin().lines();
    let t: usize = lines.next().unwrap().unwrap().parse().unwrap();

    for _ in 0..t {
        let l = lines.next().unwrap().unwrap();
        let n: usize = l.parse().unwrap();

        let s = lines.next().unwrap().unwrap();
        assert_eq!(s.len(), n);

        let ans = min_ops(&s);
        println!("{ans}");
    }
}

fn min_ops(s: &str) -> usize {
    let n = s.len();

    if n % 2 == 0 {
        return min_ops_old(s);
    }

    let mut best = usize::MAX;
    for a in 'a'..='z' {
        for b in 'a'..='z' {
            let prefix_costs = prefix_min_costs(s, a, b);
            let suffix_costs = suffix_min_costs(s, a, b);

            let mut cost = usize::MAX;
            for i in 0..n {
                // Note the +1 for killing char s[i].
                let candidate = prefix_costs[i] + 1 + suffix_costs[i + 1];
                cost = min(cost, candidate);

                // if (s, a, b, i) == ("dcbdb", 'd', 'b', 1) {
                //     println!("{:?}", prefix_costs);
                //     println!("{:?}", suffix_costs);
                // }
            }

            best = min(best, cost);
        }
    }
    best
}

/// For every prefix of s, how many even (resp. odd) chars differ from a (resp. b)?
fn prefix_min_costs(s: &str, a: char, b: char) -> Vec<usize> {
    let mut out = Vec::with_capacity(s.len() + 1);

    let mut cost = 0;
    out.push(cost);

    for (i, c) in s.chars().enumerate() {
        if i % 2 == 0 {
            if c != a {
                cost += 1;
            }
        } else {
            if c != b {
                cost += 1;
            }
        }
        out.push(cost);
    }

    out
}

/// For every suffix of s, how many even (resp. odd) chars differ from a (resp. b)?
///
/// out[0] is the whole string
/// out[n] is the empty suffix
fn suffix_min_costs(s: &str, a: char, b: char) -> Vec<usize> {
    let mut out = Vec::with_capacity(s.len() + 1);

    let mut cost = 0;
    out.push(cost);

    // Note that our notion of even/odd is flipped!
    let (a, b) = (b, a);

    for (i, c) in s.chars().rev().enumerate() {
        if i % 2 == 0 {
            if c != a {
                cost += 1;
            }
        } else {
            if c != b {
                cost += 1;
            }
        }
        out.push(cost);
    }

    out.reverse();
    out
}

// ---

/// Only works for strings with even length.
fn min_ops_old(s: &str) -> usize {
    let mut evens = String::new();
    let mut odds = String::new();
    for (i, c) in s.chars().enumerate() {
        if i % 2 == 0 {
            evens.push(c);
        } else {
            odds.push(c);
        }
    }

    let even_cost = cost_to_make_all_same(&evens);
    let odd_cost = cost_to_make_all_same(&odds);

    let cost = even_cost + odd_cost;

    // this isn't actually correct.
    // if cost == 0 && n % 2 == 1 {
    //     return 1;
    // }

    cost
}

fn cost_to_make_all_same(s: &str) -> usize {
    let n = s.len();
    if n == 0 {
        return 0;
    }

    let mut freqs = HashMap::<char, usize>::new();
    for c in s.chars() {
        *freqs.entry(c).or_default() += 1;
    }

    let max_freq = freqs.values().max().unwrap();
    n - max_freq
}
