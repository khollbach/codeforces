use std::{cmp::Ordering, io};

fn main() {
    let mut lines = io::stdin().lines();
    let t: usize = lines.next().unwrap().unwrap().parse().unwrap();

    for _ in 0..t {
        let line = lines.next().unwrap().unwrap();
        let nums: Vec<u64> = line
            .split_whitespace()
            .map(|word| word.parse().unwrap())
            .collect();

        assert_eq!(nums.len(), 2);
        let l = nums[0];
        let r = nums[1];

        let ans = longest_good_array(l, r);
        println!("{ans}");
    }
}

fn longest_good_array(l: u64, r: u64) -> u64 {
    assert!(l <= r);
    if l == r {
        return 1;
    }

    let upper_bound = r - l; // inclusive

    // goal: find the largest value of i s.t.
    // (i+2 choose 2) is <= upper_bound.
    // then return i+2.

    let mut low = 0;
    // Exclusive. Note that (10^5 choose 2) is larger than any of their test cases.
    let mut high = 100_000;

    while low < high {
        let mid = (low + high) / 2;

        match n_choose_2(mid + 2).cmp(&upper_bound) {
            Ordering::Equal => {
                return mid + 2;
            }
            Ordering::Less => low = mid + 1,
            Ordering::Greater => high = mid,
        }
    }
    debug_assert_eq!(low, high);
    let i = low - 1;

    i + 2
}

fn n_choose_2(n: u64) -> u64 {
    if n == 0 {
        return 0;
    }
    n * (n - 1) / 2
}

#[test]
fn test() {
    for (l, r, expected) in [
        (0, 0, 1),
        (0, 1, 2),
        (0, 2, 2),
        (0, 3, 3),
        (0, 4, 3),
        (0, 5, 3),
        (0, 6, 4),
        (0, 14, 5),
        (0, 15, 6),
        (0, 16, 6),
        (0, 17, 6),
    ] {
        let actual = longest_good_array(l, r);
        assert_eq!(expected, actual, "{} {}", l, r);
    }
}
