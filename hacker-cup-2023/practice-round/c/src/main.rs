use std::{io, mem};

use anyhow::Context;

fn main() -> anyhow::Result<()> {
    let mut lines = io::stdin().lines();
    let num_tests: usize = lines.next().context("num_tests")??.parse()?;

    for i in 1..=num_tests {
        let line = lines.next().context("line 1")??;
        let n: usize = line.parse()?;

        let line = lines.next().context("line 2")??;
        let mut a: Vec<_> = line
            .split_whitespace()
            .map(|s| s.parse().expect("parse"))
            .collect();
        debug_assert_eq!((2 * n).checked_sub(1), Some(a.len()));

        let ans = match smallest_purchase(&mut a) {
            Some(x) => x as i32,
            None => -1,
        };
        println!("Case #{i}: {ans}");
    }
    debug_assert!(lines.next().is_none());

    Ok(())
}

fn smallest_purchase(a: &mut [u32]) -> Option<u32> {
    assert_eq!(a.len() % 2, 1);
    if a.len() == 1 {
        return Some(1);
    }

    a.sort_unstable();

    let n = a.len();
    for i in (0..n).rev() {
        // try "deleting" a[i] -- if it works, great!
        // (we go high to low, to delete the largest possible apple.)

        let apple_to_delete = mem::replace(&mut a[i], 0);

        if let Some(weight) = pair_up(&a) {
            // We pay the remaining cost, by "buying" the missing apple.
            let cost = weight as i32 - apple_to_delete as i32;
            if cost <= 0 {
                return None; // can't buy a weightless apple :(
            }

            // TODO: yeah idk, there's a bug / edge-case in here somewhere ...

            return Some(cost as u32);
        }

        a[i] = apple_to_delete;
    }

    None
}

/// Ignore 'holes' -- entries with value 0.
/// 
/// If successful, return the pair-sum (it's the same value for all pairs).
fn pair_up(a: &[u32]) -> Option<u32> {
    assert!(!a.is_empty());

    let mut i = 0;
    let mut j = a.len() - 1;

    let mut target_sum = 0;
    target_sum += if a[i] != 0 {
        a[i]
    } else {
        a[i + 1]
    };
    target_sum += if a[j] != 0 {
        a[j]
    } else {
        a[j - 1]
    };

    while i < j {
        if a[i] == 0 {
            i += 1;
            continue;
        }
        if a[j] == 0 {
            j -= 1;
            continue;
        }

        if a[i] + a[j] != target_sum {
            return None;
        }
        i += 1;
        j -= 1;
    }

    Some(target_sum)
}
