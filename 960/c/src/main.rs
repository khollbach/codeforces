use std::{
    collections::{BTreeSet, HashSet},
    io, iter,
};

fn main() {
    let mut lines = io::stdin().lines();

    let _num_tests: usize = lines.next().unwrap().unwrap().parse().unwrap();

    while let Some(l) = lines.next() {
        let n: usize = l.unwrap().parse().unwrap();

        let a: Vec<u64> = lines
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
}

fn soln(a: &[u64]) -> u64 {
    let mut a = a.to_vec();

    let mut out = 0;

    while !a.iter().all(|&x| x == 0) {
        let sum = a.iter().sum::<u64>();
        out += sum;
        println!("{a:?} {sum}");

        a = next(&a);
        println!("{a:?} -- next");

        let (new_a, extra_sum) = optimization(&a);
        println!("{new_a:?} {extra_sum} -- opt");
        a = new_a;
        out += extra_sum;
    }

    out
}

/*
for every run of >= 2 values, cut it down to just one
and output a triangle sum (minus one copy)
*/
fn optimization(a: &[u64]) -> (Vec<u64>, u64) {
    let mut b = Vec::with_capacity(a.len());
    let mut extra_sum = 0;

    for (x, mut freq) in groups(a) {
        if freq >= 3 {
            extra_sum += x * (n_choose_2(freq as u64) - 3);
            dbg!(freq, x, extra_sum);
            freq = 2;
        }

        b.extend(iter::repeat(x).take(freq));
    }

    (b, extra_sum)
}

// (sum 1..=n) -- (is this actually called n choose 2?? and why am I so rusty that I
// don't remember that..)
fn n_choose_2(n: u64) -> u64 {
    n * (n + 1) / 2
}

fn groups(a: &[u64]) -> Vec<(u64, usize)> {
    if a.is_empty() {
        return vec![];
    }

    let mut out = Vec::with_capacity(a.len());

    let mut curr_val = a[0];
    let mut curr_freq = 1;

    for &x in &a[1..] {
        if x == curr_val {
            curr_freq += 1;
        } else {
            out.push((curr_val, curr_freq));

            curr_val = x;
            curr_freq = 1;
        }
    }

    out.push((curr_val, curr_freq));

    out
}

fn next(a: &[u64]) -> Vec<u64> {
    let n = a.len();
    let mut b = vec![0; n];

    let mut seen = HashSet::new();
    let mut dups = BTreeSet::new();

    for i in 0..n {
        if seen.contains(&a[i]) {
            dups.insert(a[i]);
        } else {
            seen.insert(a[i]);
        }

        b[i] = *dups.last().unwrap_or(&0);
    }

    b
}

/*
idea

each iteration
* apply the rule
* extract out long runs, as an optimization
* repeat

so maybe we impl the naive approach first.

*/

#[cfg(test)]
mod tests {
    use test_case::test_case;

    #[test_case(&[], &[]; "empty")]
    #[test_case(&[5], &[(5, 1)]; "just one")]
    #[test_case(&[5, 5, 5], &[(5, 3)]; "3 copies")]
    #[test_case(
        &[3, 3, 5, 2, 2, 1, 4, 1, 4, 5],
        &[(3, 2), (5, 1), (2, 2), (1, 1), (4, 1), (1, 1), (4, 1), (5, 1)];
        "general case"
    )]
    fn groups(a: &[u64], expected: &[(u64, usize)]) {
        let actual = super::groups(a);
        assert_eq!(expected, actual);
    }
}
