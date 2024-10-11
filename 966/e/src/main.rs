// EW, YUCK.
// Brutal code, did a very sloppy (trial-and-error) job of debugging value().
// But hey, we got it to work.

use std::{
    cmp::{min, Reverse},
    io,
    iter::zip,
};

fn main() {
    let mut lines = io::stdin().lines();
    let t: usize = lines.next().unwrap().unwrap().parse().unwrap();

    for _ in 0..t {
        let l = lines.next().unwrap().unwrap();
        let nmk: Vec<i64> = l
            .split_whitespace()
            .map(|word| word.parse().unwrap())
            .collect();
        let n = nmk[0];
        let m = nmk[1];
        let k = nmk[2];

        let l = lines.next().unwrap().unwrap();
        let w = l.parse().unwrap();

        let l = lines.next().unwrap().unwrap();
        let mut heights: Vec<i64> = l
            .split_whitespace()
            .map(|word| word.parse().unwrap())
            .collect();
        assert_eq!(heights.len(), w);

        let ans = soln(n, m, k, &mut heights);
        println!("{ans}");
    }
}

fn soln(n: i64, m: i64, k: i64, heights: &mut [i64]) -> i64 {
    let mut multipliers = Vec::with_capacity((n * m) as usize);

    for i in 0..n {
        for j in 0..m {
            let i_dist = 1 + min(i, n - 1 - i);
            let j_dist = 1 + min(j, m - 1 - j);

            // print!("{}\t", value(n, m, k, i, j)); // TODO

            // let value = min(i_dist, k) * min(j_dist, k);
            let value = value(n, m, k, i, j); // TODO
            multipliers.push(value);
        }
        // println!();
    }

    heights.sort_by_key(|&h| Reverse(h));
    multipliers.sort_by_key(|&mult| Reverse(mult));

    let mut score = 0;
    for (h, mult) in zip(heights, multipliers) {
        // dbg!(*h, mult);
        score += *h * mult;
    }
    score
}

#[test]
fn test() {
    let ans = value(9, 5, 5, 8, 0);
    dbg!(ans);
    panic!();
}

fn value(n: i64, m: i64, k: i64, i: i64, j: i64) -> i64 {
    let mut width = k;

    let left_sub = j - (k - 1);
    if left_sub < 0 {
        width += left_sub;
    }

    let right_sub = m - (j + k);
    if right_sub < 0 {
        width += right_sub;
    }

    let mut height = k;

    let up = i - (k - 1);
    if up < 0 {
        height += up;
    }

    let down = n - (i + k);
    if down < 0 {
        height += down;
    }

    // dbg!(left_sub, right_sub, up, down);
    // dbg!(width, height);

    min(width, k) * min(height, k)
}
