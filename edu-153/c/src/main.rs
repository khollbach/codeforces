use std::{cmp::min, error::Error, io, result::Result as StdResult};

type Result<T> = StdResult<T, Box<dyn Error>>;

fn main() -> Result<()> {
    let mut lines = io::stdin().lines();
    let _num_tests: usize = lines.next().ok_or("no input")??.parse()?;

    while let Some(line) = lines.next() {
        let n = line?.parse()?;

        let nums: Vec<_> = lines
            .next()
            .ok_or("expected nums")??
            .split_whitespace()
            .map(str::parse)
            .collect::<StdResult<_, _>>()?;
        assert_eq!(nums.len(), n);

        let ans = num_winning(&nums);
        println!("{ans}");
    }

    Ok(())
}

fn num_winning(perm: &[u32]) -> usize {
    let n = perm.len();

    // A number is "poisoned" if taking it ends the game immediately.

    let mut poisoned = Vec::with_capacity(n);
    let mut smallest_seen = u32::MAX; // infty

    for &x in perm {
        if x < smallest_seen {
            // New smallest; poisoned!
            poisoned.push(true);
        } else {
            // Safe; there's something smaller to the left of us.
            poisoned.push(false);
        }

        smallest_seen = min(smallest_seen, x);
    }

    // A number is "losing" if there's a smaller, *non-poisoned* number
    // to the left of it.
    //
    // (If you pick a "losing" number, the other player will pick the
    // *left-most* smaller non-poisoned number. You must then eat poison.)

    let mut losing = Vec::with_capacity(n);
    let mut smallest_non_poisoned = u32::MAX; // infty

    for (i, &x) in perm.iter().enumerate() {
        if smallest_non_poisoned < x {
            // L.
            losing.push(true);
        } else {
            // Not "losing" in the sense described above.
            losing.push(false);
        }

        if !poisoned[i] {
            smallest_non_poisoned = min(smallest_non_poisoned, x);
        }
    }

    // All other numbers are winning. The second player has no choice but to eat poison.

    (0..n).filter(|&i| !poisoned[i] && !losing[i]).count()
}
