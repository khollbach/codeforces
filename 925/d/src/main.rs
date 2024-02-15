use std::{collections::HashMap, io};

fn main() {
    let lines: Vec<_> = io::stdin().lines().skip(1).map(Result::unwrap).collect();
    for nxy_nums in lines.chunks(2) {
        let [nxy, nums] = &nxy_nums[..] else { panic!() };

        let nxy: Vec<u32> = nxy.split_whitespace().map(|s| s.parse().unwrap()).collect();
        let &[_n, x, y] = &nxy[..] else { panic!() };

        let nums: Vec<u32> = nums
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        let ans = soln(&nums, x, y);
        println!("{}", ans);
    }
}

/// number of good pairs
fn soln(nums: &[u32], x: u32, y: u32) -> usize {
    let mut out = 0;

    // (a%x, a%y) -> count
    let mut map = Map::new();

    for &a in nums {
        out += map
            .get(&(compliment(a, x), a % y))
            .copied()
            .unwrap_or_default();
        *map.entry((a % x, a % y)).or_default() += 1;
    }

    out
}

type Map = HashMap<(u32, u32), usize>;

fn compliment(a: u32, x: u32) -> u32 {
    if a % x == 0 {
        0
    } else {
        x - a % x
    }
}
