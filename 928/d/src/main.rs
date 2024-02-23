use std::{cmp::min, collections::HashMap, io};

fn main() {
    let mut lines = io::stdin().lines().skip(1).map(Result::unwrap);
    while let Some(_n) = lines.next() {
        let nums: Vec<u32> = lines
            .next()
            .unwrap()
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        let ans = soln(&nums);
        println!("{ans}");
    }
}

/* idea
- collect up the numbers into a multiset
- iterate thru the original numbers:
    - let num_pairs = min(freq[x], freq[~x])
    - freq[x] -= num_pairs;
    - freq[~x] -= num_pairs;
    - ans += num_pairs;
    - ans += freq[x];
    - ans += freq[~x];
    - freq[x] = 0;
    - freq[~x] = 0;
*/

fn soln(nums: &[u32]) -> usize {
    let mut freqs = HashMap::<u32, usize>::new();
    for &x in nums {
        *freqs.entry(x).or_default() += 1;
    }
    // dbg!(&freqs);

    let mut ans = 0;

    for &x in nums {
        let num_pairs = min(
            *freqs.entry(x).or_default(),
            *freqs.entry(flip(x)).or_default(),
        );

        ans += num_pairs;
        *freqs.entry(x).or_default() -= num_pairs;
        *freqs.entry(flip(x)).or_default() -= num_pairs;

        ans += *freqs.entry(x).or_default();
        ans += *freqs.entry(flip(x)).or_default();
        freqs.insert(x, 0);
        freqs.insert(flip(x), 0);

        // dbg!(x, num_pairs, ans);
    }

    ans
}

// x: u31
// flip all the bits of x to get a new u31
fn flip(x: u32) -> u32 {
    let high_bit = 1 << 31;
    let mask = !high_bit;
    !x & mask // keep the high bit clear
}
