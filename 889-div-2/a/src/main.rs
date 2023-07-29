use std::{error::Error, io, result::Result as StdResult};

type Result<T> = StdResult<T, Box<dyn Error>>;

fn main() -> Result<()> {
    let mut lines = io::stdin().lines();
    let _num_tests: usize = lines.next().ok_or("no input")??.parse()?;

    while let Some(line) = lines.next() {
        let n = line?.parse()?;

        let mut nums: Vec<_> = lines
            .next()
            .ok_or("expected line of nums")??
            .split_whitespace()
            .map(str::parse)
            .collect::<StdResult<_, _>>()?;
        debug_assert_eq!(nums.len(), n);

        // 0-indexed, please.
        for x in &mut nums {
            assert_ne!(*x, 0);
            *x -= 1;
            debug_assert!((*x as usize) < n);
        }

        let ans = soln(&nums);
        println!("{ans}");
    }

    Ok(())
}

/// Min swaps to completely scramble (no number matches its index).
fn soln(nums: &[u32]) -> usize {
    // Impossible if n is 1.
    assert!(nums.len() != 1);

    let num_matches = nums
        .iter()
        .enumerate()
        .filter(|&(i, &x)| i == x as usize)
        .count();
    ceil_div(num_matches, 2)
}

fn ceil_div(a: usize, b: usize) -> usize {
    let extra = (a % b != 0) as usize;
    a / b + extra
}
