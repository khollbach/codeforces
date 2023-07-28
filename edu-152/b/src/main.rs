use std::{cmp::Reverse, error::Error, io, result::Result as StdResult};

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
            Err("expected 2 nums: n and k")?
        };

        let nums: Vec<_> = lines
            .next()
            .ok_or("expected line of monster healths")??
            .split_whitespace()
            .map(str::parse)
            .collect::<StdResult<_, _>>()?;
        debug_assert_eq!(nums.len(), n);

        let ans = soln(&nums, k as u32);

        for (first, idx) in ans.into_iter().enumerate() {
            if first != 0 {
                print!(" ");
            }

            let one_based = idx + 1;
            print!("{one_based}");
        }
        println!();
    }

    Ok(())
}

fn soln(monster_healths: &[u32], ability_damage: u32) -> Vec<usize> {
    assert_ne!(ability_damage, 0);

    // Hit monsters until they're all one hit from death.
    let mut low_healths: Vec<(usize, u32)> = monster_healths
        .iter()
        .map(|&health| {
            assert_ne!(health, 0);

            let health_remaining = health % ability_damage;
            if health_remaining == 0 {
                // Don't kill it; leave it at a round-number of health.
                ability_damage
            } else {
                health_remaining
            }
        })
        .enumerate()
        .collect();

    // Kill them all, most-healthy-first. Break ties by lowest-index-first.
    low_healths.sort_unstable_by_key(|&(idx, health)| (Reverse(health), idx));

    // What order do they die in?
    low_healths.into_iter().map(|(idx, _)| idx).collect()
}
