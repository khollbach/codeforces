use std::{cmp::max, io};

fn main() {
    let lines: Vec<_> = io::stdin().lines().skip(1).map(Result::unwrap).collect();
    for nl in lines.chunks(2) {
        let [_n, l] = &nl[..] else { panic!() };
        let mut nums: Vec<u32> = l.split_whitespace().map(|s| s.parse().unwrap()).collect();
        let ans = soln(&mut nums);
        println!("{}", ans);
    }
}

/// Minimum cost to make them all the same.
fn soln(nums: &[u32]) -> usize {
    let n = nums.len();
    assert_ne!(n, 0);

    // degenerate case: all the same
    if nums.iter().all(|&x| x == nums[0]) {
        return 0;
    }

    // islands are disjoint
    let left_island = nums.iter().take_while(|&&x| x == nums[0]).count();
    let right_island = nums.iter().rev().take_while(|&&x| x == nums[n - 1]).count();

    if nums[0] == nums[n - 1] {
        n - left_island - right_island
    } else {
        let larger = max(left_island, right_island);
        n - larger
    }
}
