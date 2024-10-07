use std::{error::Error, io};

type Result<T> = std::result::Result<T, Box<dyn Error>>;

fn main() -> Result<()> {
    let mut lines = io::stdin().lines();
    let t: usize = lines.next().ok_or("t")??.parse()?;

    for _ in 0..t {
        let l = lines.next().ok_or("l")??;
        let k: usize = l.parse()?;

        let ans = soln(k);
        println!("{ans}");
    }

    assert!(lines.next().is_none());

    Ok(())
}

// ok, 10^18 is a large enough upper bound
// fn main() {
//     let n = 10_u64.pow(18);
//     let tmp = living_index(&n.to_string());
//     dbg!(tmp);
// }

fn soln(target: usize) -> u64 {
    // TODO: name
    let n = find_number(target);
    increase_until_live(n)
}

/// Find the smallest live number that is >= n.
fn increase_until_live(n: u64) -> u64 {
    let s = n.to_string();
    if !s.contains("4") {
        return n;
    }

    let first_four = s.find("4").unwrap();
    let trailing_zeros = s.len() - (first_four + 1);

    let mut out = String::with_capacity(s.len());
    out.push_str(&s[..first_four]);
    out.push_str("5");
    out.push_str(&"0".repeat(trailing_zeros));
    out.parse().unwrap()
}

/// Might return a dead number below the actual number we're looking for. (TODO: explain better)
fn find_number(target: usize) -> u64 {
    // TODO: name
    let mut low = 0;
    let mut high = 10_u64.pow(18); // exclusive

    while low < high {
        let mid = (low + high) / 2;

        let key = living_index(&mid.to_string());
        if target < key {
            high = mid;
        } else if target > key {
            low = mid + 1;
        } else {
            return mid;
        }
    }
    debug_assert_eq!(low, high);

    panic!("not found");
}

/// How many "live" numbers less than n?
///
/// A live number is one with no '4' digit.
fn living_index(n: &str) -> usize {
    assert!(n.chars().all(|c| c.is_ascii_digit()));
    if n.is_empty() {
        return 0;
    }

    let mut total = 0;

    let first_digit = n.chars().next().unwrap();
    for d in '0'..first_digit {
        if d != '4' {
            total += 9_usize.pow(n.len() as u32 - 1);
        }
    }

    if first_digit != '4' {
        total += living_index(&n[1..]);
    }

    total
}
