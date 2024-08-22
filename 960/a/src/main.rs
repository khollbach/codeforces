use std::{collections::HashMap, io};

fn main() {
    let mut lines = io::stdin().lines();

    let _num_tests: usize = lines.next().unwrap().unwrap().parse().unwrap();

    while let Some(l) = lines.next() {
        let n = l.unwrap().parse().unwrap();

        let a: Vec<u8> = lines
            .next()
            .unwrap()
            .unwrap()
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        assert_eq!(a.len(), n);

        let ans = soln(&a);
        if ans {
            println!("Yes");
        } else {
            println!("No");
        }
    }
}

/// Can Alice win?
///
/// If there's any element with odd parity, then yes.
fn soln(a: &[u8]) -> bool {
    let mut freqs = HashMap::new();
    for x in a {
        *freqs.entry(x).or_default() += 1;
    }

    for f in freqs.values() {
        if f % 2 != 0 {
            return true;
        }
    }
    false
}
