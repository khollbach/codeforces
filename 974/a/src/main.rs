use std::{error::Error, io};

type Result<T> = std::result::Result<T, Box<dyn Error>>;

fn main() -> Result<()> {
    let mut lines = io::stdin().lines();
    let t: usize = lines.next().ok_or("t")??.parse()?;

    for _ in 0..t {
        let nk = lines.next().ok_or("nk")??;
        let nk: Vec<u32> = nk
            .split_whitespace()
            .map(str::parse)
            .collect::<std::result::Result<_, _>>()?;
        let &[n, k] = nk.as_slice() else {
            panic!("expected 2 nums nk, got: {}", nk.len());
        };

        let nums = lines.next().ok_or("nums")??;
        let nums: Vec<u32> = nums
            .split_whitespace()
            .map(str::parse)
            .collect::<std::result::Result<_, _>>()?;
        assert_eq!(nums.len(), n as usize);

        let ans = num_donations(k, &nums);
        println!("{ans}");
    }

    assert!(lines.next().is_none());
    Ok(())
}

fn num_donations(richness_threshold: u32, gold: &[u32]) -> usize {
    assert_ne!(richness_threshold, 0);

    let mut count = 0;
    let mut stash = 0;

    for &g in gold {
        if g >= richness_threshold {
            // Steal.
            stash += g;
        } else if g == 0 && stash > 0 {
            // Donate.
            stash -= 1;
            count += 1;
        }
    }

    count
}
