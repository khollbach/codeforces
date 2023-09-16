use std::{error::Error, io, result::Result as StdResult};

type Result<T> = StdResult<T, Box<dyn Error>>;

fn main() -> Result<()> {
    let mut lines = io::stdin().lines();
    let _num_tests: usize = lines.next().ok_or("no input")??.parse()?;

    while let Some(line) = lines.next() {
        let nxy: Vec<_> = line?
            .split_whitespace()
            .map(str::parse)
            .collect::<StdResult<_, _>>()?;
        let &[n, x, y] = nxy.as_slice() else {
            Err("expected 3 nums")?
        };

        let ans = soln(n, x, y);
        println!("{ans}");
    }

    Ok(())
}

fn soln(n: i64, x: i64, y: i64) -> i64 {
    // dbg!(n, x, y);
    assert!(n > 0);
    assert!(x > 0);
    assert!(y > 0);

    // How many "red" spaces, and how many "blue" ?
    let num_x = num_divisible(n, x);
    let num_y = num_divisible(n, y);
    let num_both = num_divisible(n, lcm(x, y)); //x * y); // LCM !
    let only_x = num_x - num_both;
    let only_y = num_y - num_both;
    // dbg!(num_x, num_y, num_both, only_x, only_y);

    let mut ans = 0;

    // greedy thing:
    // use the largest numbers in red spaces
    ans += sum_thru(n) - sum_thru(n - only_x);
    // ans += dbg!(dbg!(sum_thru(n)) - dbg!(sum_thru(n - only_x)));

    // use the smallest numbers in blue spaces
    ans -= sum_thru(only_y);

    ans
}

fn lcm(x: i64, y: i64) -> i64 {
    assert!(x > 0);
    assert!(y > 0);
    x * y / gcd(x, y)
}

fn gcd(x: i64, y: i64) -> i64 {
    assert!(x >= 0);
    assert!(y >= 0);

    // wlog x <= y
    let (x, y) = if x <= y { (x, y) } else { (y, x) };

    if x == 0 {
        return y;
    }

    let z = y % x;
    gcd(x, z)
}

/// compute sum(1..=n)
fn sum_thru(n: i64) -> i64 {
    assert!(n >= 0);

    n * (n + 1) / 2
}

/// How many numbers in 1..=n are divisible by x?
fn num_divisible(n: i64, x: i64) -> i64 {
    assert!(n > 0);
    assert!(x > 0);

    n / x

    // // first count in 0..n:
    // let mut ans = n / x;

    // // then fix up edge cases
    // // 0:
    // ans -= 1;
    // // n:
    // if n % x == 0 {
    //     ans += 1;
    // }

    // ans
}
