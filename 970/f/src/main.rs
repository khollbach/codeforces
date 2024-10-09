use std::io;

fn main() {
    let mut lines = io::stdin().lines();
    let t: usize = lines.next().unwrap().unwrap().parse().unwrap();

    for _ in 0..t {
        let n: usize = lines.next().unwrap().unwrap().parse().unwrap();
        let nums: Vec<u128> = lines
            .next()
            .unwrap()
            .unwrap()
            .split_whitespace()
            .map(|word| word.parse().unwrap())
            .collect();
        assert_eq!(nums.len(), n);

        let ans = expected_value_of_product(&nums);
        println!("{ans}");
    }
}

fn expected_value_of_product(nums: &[u128]) -> u128 {
    let n = nums.len();
    let nums_sum: u128 = nums.iter().sum();

    let mut pairs_sum = 0;
    for i in 0..n {
        pairs_sum += nums[i] * nums_sum;
    }

    for i in 0..n {
        pairs_sum -= nums[i] * nums[i];
    }

    let num_pairs = n as u128 * n as u128 - n as u128;

    reduce(pairs_sum, num_pairs)
}

const MOD: u128 = 1_000_000_007; // 10^9 + 7

/// Reduce the ratio `x / y`, mod 10^9+7
fn reduce(x: u128, y: u128) -> u128 {
    assert_ne!(y, 0);
    x % MOD * inverse(y) % MOD
}

fn inverse(mut a: u128) -> u128 {
    let p = MOD;
    let mut ex = p - 2;
    let mut result = 1;
    while ex > 0 {
        if ex % 2 == 1 {
            result = (result * a) % p;
        }
        a = (a * a) % p;
        ex /= 2;
    }
    result
}
