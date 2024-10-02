use std::{cmp::min, error::Error, io};

type Result<T> = std::result::Result<T, Box<dyn Error>>;

fn main() -> Result<()> {
    let mut lines = io::stdin().lines();
    let t: usize = lines.next().ok_or("t")??.parse()?;

    for _ in 0..t {
        let l = lines.next().ok_or("n")??;
        let (n, k) = parse_pair(&l)?;
        let ans = closest_split(n, k);
        println!("{ans}");
    }

    if lines.next().is_some() {
        return Err("expected EOF".into());
    }

    Ok(())
}

/// Return the smallest difference, according to the rules of the problem.
fn closest_split(n: i64, k: i64) -> i64 {
    assert!(n >= 2);
    assert!(k >= 2);

    let mut low = k;
    let mut high = k + n; // exclusive

    while low < high {
        let mid = (low + high) / 2;

        let left_sum = sum_range(k, mid - 1);
        let right_sum = sum_range(mid, k + n - 1);
        if left_sum < right_sum {
            low = mid + 1;
        } else if left_sum > right_sum {
            high = mid;
        } else {
            return 0;
        }
    }
    debug_assert_eq!(low, high);

    let split_1 = (sum_range(k, low - 1) - sum_range(low, k + n - 1)).abs();
    let split_2 = (sum_range(k, low - 2) - sum_range(low - 1, k + n - 1)).abs();

    min(split_1, split_2)
}

/// Inclusive of both endpoints.
fn sum_range(a: i64, b: i64) -> i64 {
    if b < a {
        return 0;
    }
    sum_1_through_n(b) - sum_1_through_n(a - 1)
}

fn sum_1_through_n(n: i64) -> i64 {
    (0..=n).sum()
}

fn parse_pair(l: &str) -> Result<(i64, i64)> {
    let nums: Vec<i64> = l
        .split_whitespace()
        .map(str::parse)
        .collect::<std::result::Result<_, _>>()?;
    let &[x, y] = nums.as_slice() else {
        return Err(format!("expected 2 nums, got: {}", nums.len()).into());
    };
    Ok((x, y))
}
