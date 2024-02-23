use std::io;

fn main() {
    let mut lines = io::stdin().lines().skip(1).map(Result::unwrap);
    while let Some(_n) = lines.next() {
        let nums: Vec<u32> = lines
            .next()
            .unwrap()
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        let ans = soln(&nums);
        println!("{}", ans);
    }
}

fn soln(nums: &[u32]) -> u32 {
    let mut curr_year = 0;

    for &x in nums {
        // find the smallest multiple of x that is strictly greater than curr_year.
        let rounded_down = curr_year / x * x;
        curr_year = rounded_down + x;
    }

    curr_year
}
