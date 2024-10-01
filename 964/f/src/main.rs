use std::{error::Error, io};

type Result<T> = std::result::Result<T, Box<dyn Error>>;

fn main() -> Result<()> {
    let mut lines = io::stdin().lines();
    let t: usize = lines.next().ok_or("t")??.parse()?;

    for _ in 1..=t {
        let line = lines.next().ok_or("l")??;
        let (n, k) = parse_nk(&line)?;

        let line = lines.next().ok_or("l")??;
        let num_ones = count_ones(&line)?;

        let f = Factorial::new(n);
        let ans = majority_one_subsets(n, k, num_ones, &f);
        println!("{ans}");
    }

    if lines.next().is_some() {
        return Err("expected EOF".into());
    }

    Ok(())
}

const N: u64 = 1_000_000_007; // 10^9 + 7

/// How many subsets of size k have more than k/2 ones?
fn majority_one_subsets(n: u64, k: u64, num_ones: u64, f: &Factorial) -> u64 {
    let num_zeros = n - num_ones;

    let mut ways = 0;
    for i in ceil_div(k, 2)..=k {
        // Count the number of ways to take exactly `i` many ones.
        ways += choose(num_ones, i, f) * choose(num_zeros, k - i, f);
        ways %= N;
    }
    ways
}

fn choose(x: u64, y: u64, f: &Factorial) -> u64 {
    if y > x {
        return 0;
    }

    let a = f.get(x);
    let b = f.get(x - y);
    let c = f.get(y);

    div(div(a, b), c)
}

fn div(a: u64, b: u64) -> u64 {
    a * inverse(b) % N
}

/// https://stackoverflow.com/questions/14093417/find-the-inverse-of-a-number-modulo-a-prime
fn inverse(mut a: u64) -> u64 {
    let p = N;

    // Compute a^(p-2) by fast exponentiation: repeatedly square a, and include
    // the values that correspond to 1s in the binary representation of p-2.
    let mut ex = p - 2;
    let mut out = 1;

    while ex > 0 {
        if ex % 2 == 1 {
            out *= a;
            out %= p;
        }

        a *= a;
        a %= p;

        ex /= 2;
    }

    out
}

struct Factorial {
    cache: Vec<u64>,
}

impl Factorial {
    /// Cache values of i! for i in 0..=n.
    fn new(n: u64) -> Self {
        let mut cache = vec![0; n as usize + 1];

        // Edge-case: 0! == 1.
        cache[0] = 1;

        let mut p = 1;
        for i in 1..=n {
            p *= i;
            p %= N;
            cache[i as usize] = p;
        }

        Self { cache }
    }

    fn get(&self, x: u64) -> u64 {
        self.cache[x as usize]
    }
}

fn ceil_div(x: u64, y: u64) -> u64 {
    assert_ne!(y, 0);
    let extra = if x % y != 0 { 1 } else { 0 };
    x / y + extra
}

fn parse_nk(l: &str) -> Result<(u64, u64)> {
    let nk: Vec<_> = l
        .split_whitespace()
        .map(str::parse)
        .collect::<std::result::Result<_, _>>()?;
    let &[n, k] = nk.as_slice() else {
        return Err(format!("nk.len(): {}", nk.len()).into());
    };
    Ok((n, k))
}

fn count_ones(line: &str) -> Result<u64> {
    let mut count = 0;
    for s in line.split_whitespace() {
        match s {
            "0" => (),
            "1" => count += 1,
            _ => panic!("{s}"),
        }
    }
    Ok(count)
}
