use std::{error::Error, io, result::Result as StdResult};

type Result<T> = StdResult<T, Box<dyn Error>>;

fn main() -> Result<()> {
    let mut lines = io::stdin().lines();
    let _num_tests: usize = lines.next().ok_or("no input")??.parse()?;

    while let Some(line) = lines.next() {
        let line = line?;
        let nk: Vec<_> = line.split_whitespace().collect();
        let &[n, k] = nk.as_slice() else {
            Err("expected two words: n and k")?
        };

        let mut nums: Vec<_> = lines
            .next()
            .ok_or("expected line of nums")??
            .split_whitespace()
            .map(str::parse)
            .collect::<StdResult<_, _>>()?;
        if nums.len() != n.parse()? {
            Err("expected n nums")?
        }

        let ans = deletions_required(&mut nums, k.parse()?);
        println!("{ans}");
    }

    Ok(())
}

fn deletions_required(nums: &mut [u32], k: u32) -> usize {
    nums.sort_unstable();

    let gaps: Vec<_> = nums.windows(2).map(|pair| pair[1] - pair[0]).collect();

    let longest_good_island = gaps
        .split(|&x| x > k)
        .map(|slice| slice.len())
        .max()
        .unwrap_or(0);

    gaps.len() - longest_good_island
}
