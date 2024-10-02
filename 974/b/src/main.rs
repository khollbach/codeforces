use std::{cmp::max, error::Error, io};

type Result<T> = std::result::Result<T, Box<dyn Error>>;

fn main() -> Result<()> {
    let mut lines = io::stdin().lines();
    let t: usize = lines.next().ok_or("t")??.parse()?;

    for _ in 0..t {
        let nk = lines.next().ok_or("nk")??;
        let nk: Vec<i32> = nk
            .split_whitespace()
            .map(str::parse)
            .collect::<std::result::Result<_, _>>()?;
        let &[n, k] = nk.as_slice() else {
            panic!("expected 2 nums nk, got: {}", nk.len());
        };

        let ans = num_odd_years(n, k);
        if ans % 2 == 0 {
            println!("YES");
        } else {
            println!("NO");
        }
    }

    assert!(lines.next().is_none());
    Ok(())
}

fn num_odd_years(n: i32, k: i32) -> i32 {
    let low = max(1, n - k + 1);
    let high = n;
    num_odd_numbers_in_range(low, high)
}

fn num_odd_numbers_in_range(low: i32, high_inclusive: i32) -> i32 {
    if low % 2 == 0 {
        (high_inclusive + 1 - low) / 2
    } else {
        ceil_div(high_inclusive + 1 - low, 2)
    }
}

fn ceil_div(a: i32, b: i32) -> i32 {
    assert_ne!(b, 0);
    let extra = (a.rem_euclid(b) != 0) as i32;
    a / b + extra
}
