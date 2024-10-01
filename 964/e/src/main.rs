use std::{error::Error, io};

type Result<T> = std::result::Result<T, Box<dyn Error>>;

fn main() -> Result<()> {
    let mut lines = io::stdin().lines();
    let t: usize = lines.next().ok_or("t")??.parse()?;

    for _ in 1..=t {
        let line = lines.next().ok_or("l")??;
        let (l, r) = parse_line(&line)?;

        let ans = time_to_zero_range(l, r);
        println!("{ans}");
    }

    if lines.next().is_some() {
        return Err("expected EOF".into());
    }

    Ok(())
}

fn parse_line(l: &str) -> Result<(u32, u32)> {
    let lr: Vec<_> = l
        .split_whitespace()
        .map(str::parse)
        .collect::<std::result::Result<_, _>>()?;
    let &[l, r] = lr.as_slice() else {
        return Err(format!("lr.len(): {}", lr.len()).into());
    };
    Ok((l, r))
}

fn time_to_zero_range(l: u32, r: u32) -> u32 {
    let mut total = 0;

    // The first number costs us twice as much, because it forces us to grow
    // another number that many times.
    total += 2 * (log_3(l) + 1);

    // TOO SLOW!
    // for i in l + 1..=r {
    //     total += (log_3(i) + 1);
    // }
    total += time_to_zero_all(r) - time_to_zero_all(l);

    total
}

fn time_to_zero_all(n: u32) -> u32 {
    if n == 0 {
        return 0;
    }
    let mut time = 0;

    let log = log_3(n);
    let len = log + 1;
    let high_bit = 3_u32.pow(log);

    time += len * (n - high_bit + 1);

    for i in 1..len {
        time += how_many_nums_with_len(i) * i;
    }

    time
}

fn how_many_nums_with_len(len: u32) -> u32 {
    assert_ne!(len, 0);
    3_u32.pow(len) - 3_u32.pow(len - 1)
}

/// Log base 3, rounded down to the nearest integer.
///
/// Equal to the index of the highest ternary-bit.
fn log_3(mut n: u32) -> u32 {
    assert_ne!(n, 0);
    for i in 0.. {
        n /= 3;
        if n == 0 {
            return i;
        }
    }
    unreachable!();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_log_3() {
        for (n, expected) in [(1, 0), (2, 0), (3, 1), (4, 1), (8, 1), (9, 2)] {
            let actual = log_3(n);
            assert_eq!(expected, actual);
        }
    }

    #[test]
    fn test_time_to_zero_all() {
        for n in [1, 2, 3, 4, 8, 9] {
            let expected = time_to_zero_all_slow(n);
            let actual = time_to_zero_all(n);
            assert_eq!(expected, actual, "{n}");
        }
    }

    fn time_to_zero_all_slow(n: u32) -> u32 {
        (1..=n).map(|i| log_3(i) + 1).sum()
    }

    #[test]
    fn test_how_many_nums_with_len() {
        for (len, expected) in [
            (1, 2),
            (2, 6),
            (3, 18),
        ] {
            let actual = how_many_nums_with_len(len);
            assert_eq!(expected, actual, "{len}");
        }
    }
}
