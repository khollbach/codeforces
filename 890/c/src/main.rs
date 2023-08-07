use std::{error::Error, io, result::Result as StdResult};

type Result<T> = StdResult<T, Box<dyn Error>>;

fn main() -> Result<()> {
    let mut lines = io::stdin().lines();
    let _num_tests: usize = lines.next().ok_or("no input")??.parse()?;

    while let Some(line) = lines.next() {
        let nk: Vec<_> = line?
            .split_whitespace()
            .map(str::parse)
            .collect::<StdResult<_, _>>()?;
        let &[n, k] = nk.as_slice() else {
            Err("expected 2 nums: n k")?
        };

        let nums: Vec<_> = lines
            .next()
            .ok_or("expected line of nums")??
            .split_whitespace()
            .map(str::parse)
            .collect::<StdResult<_, _>>()?;
        debug_assert_eq!(nums.len(), n as usize);

        let ans = largest_number_after_k_ops(&nums, k);
        println!("{ans}");
    }

    Ok(())
}

// O(n^2)
fn largest_number_after_k_ops(a: &[u32], k: u32) -> u32 {
    assert!(a.len() >= 2);
    assert!(k >= 1);

    // Try all non-empty suffixes.
    (0..a.len())
        .map(|i| {
            let mut suffix: Vec<_> = a[i..].iter().map(|&x| x as i32).collect();
            largest_a0_after_k_ops(&mut suffix, k as i32) as u32
        })
        .max()
        .unwrap()
}

// O(n)
fn largest_a0_after_k_ops(a: &mut [i32], mut k: i32) -> i32 {
    for i in 0..a.len() - 1 {
        // Invariant: a[0..=i] is a downward-staircase.
        debug_assert!(a[0..=i].windows(2).all(|pair| pair[0] - 1 == pair[1]));

        // Goal: make a[i+1] part of the staircase.
        let delta = a[i + 1] - a[i];

        if delta == -1 {
            // Good. Our staircase is growing!
            continue;
        } else if delta < -1 {
            // Increase a[i+1] to make it part of the staircase.

            let amount_to_add = delta.abs() - 1;
            if amount_to_add > k {
                break;
            }

            k -= amount_to_add;
            a[i + 1] += amount_to_add;
        }
        // delta >= 0
        else {
            // Increase the whole staircase a[0..=i] as much as possible. If we
            // have enough cash, this will make a[i+1] part of the staircase.

            let staircase_len = i as i32 + 1;

            let max_amount_to_add = delta + 1;
            let amount_to_add = max_amount_to_add.clamp(0, k / staircase_len);

            // Add the amount to each of a[0..=i].
            k -= amount_to_add * staircase_len;
            a[i] += amount_to_add;
            if i != 0 {
                // Optimization: ignore a[1..i].
                a[0] += amount_to_add;
            }
            // Uphold the invariant, for debugging.
            #[cfg(debug_assertions)]
            for j in 1..i {
                a[j] += amount_to_add;
            }

            // We can't extend the staircase any further, since we ran out of
            // cash.
            if amount_to_add != max_amount_to_add {
                break;
            }
        }
    }

    a[0]
}
