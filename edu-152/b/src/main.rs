use std::{collections::BinaryHeap, error::Error, io, result::Result as StdResult, cmp::{Ordering, Reverse}};

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

#[derive(PartialEq, Eq)]
struct Monster {
    curr_health: u32,
    index: usize,
}

impl Ord for Monster {
    fn cmp(&self, other: &Self) -> Ordering {
        let k1 = (self.curr_health, Reverse(self.index));
        let k2 = (other.curr_health, Reverse(other.index));
        k1.cmp(&k2)
    }
}

impl PartialOrd for Monster {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn soln(monster_healths: &[u32], ability_damage: u32) -> Vec<usize> {
    let mut q: BinaryHeap<_> = monster_healths
        .iter()
        .enumerate()
        .map(|(index, &curr_health)| Monster { index, curr_health })
        .collect();

    let mut ans = Vec::with_capacity(q.len());

    while let Some(mut m) = q.pop() {
        // Attack the highest health monster.
        m.curr_health = m.curr_health.saturating_sub(ability_damage);

        if m.curr_health == 0 {
            ans.push(m.index);
        } else {
            q.push(m)
        }
    }

    ans
}
