use std::{error::Error, io, result::Result as StdResult};

type Result<T> = StdResult<T, Box<dyn Error>>;

fn main() -> Result<()> {
    let mut lines = io::stdin().lines();
    let n: usize = lines.next().ok_or("no input")??.parse()?;

    let nums: Vec<_> = lines
        .next()
        .ok_or("expected line of nums")??
        .split_whitespace()
        .map(str::parse)
        .collect::<StdResult<_, _>>()?;
    debug_assert_eq!(nums.len(), n);

    let ans = soln(nums);
    println!("{ans}");

    Ok(())
}

fn soln(mut nums: Vec<u8>) -> usize {
    let n = nums.len();
    let islands = islands(&nums);

    // Pay 1 coin for each island.
    let mut cost = islands.len();

    for (i, j) in islands {
        // Bleed paint to both sides.
        if nums[i..j].contains(&2) {
            if i != 0 {
                nums[i - 1] = 1;
            }
            if j != n {
                nums[j] = 1;
            }
        }
        // Bleed paint to only one side.
        // Prefer the left side, unless it's already covered.
        // This greedy approach should ensure bled-paint never goes to waste.
        else {
            if i != 0 && nums[i - 1] == 0 {
                nums[i - 1] = 1;
            } else if j != n {
                nums[j] = 1;
            }
        }
    }

    // Pay 1 coin for each remaining 0.
    cost += nums.into_iter().filter(|&x| x == 0).count();

    cost
}

fn islands(nums: &[u8]) -> Vec<(usize, usize)> {
    let n = nums.len();
    let mut out = vec![];

    let mut indices = 0..n;

    while let Some(start) = indices.find(|&i| nums[i] != 0) {
        let end = indices.find(|&i| nums[i] == 0).unwrap_or(n);

        out.push((start, end));
    }

    out
}
