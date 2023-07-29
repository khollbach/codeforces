use std::{cmp::max, error::Error, io, result::Result as StdResult};

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

    let ans = soln(&nums);
    println!("{ans}");

    Ok(())
}

/// This seems to confirm that the judge compiles our code in release mode.
fn _debug_assert_soln(_nums: &[u32]) -> u32 {
    // When we submit this code, the assert doesn't fire.
    debug_assert!(false);

    0
}

fn soln(nums: &[u32]) -> u32 {
    let n = nums.len();
    assert_ne!(n, 0);

    // dp[i][j] is the subproblem where:
    // * nums[0..i] are ignored,
    // * nums[i..j] are unlocked, and
    // * nums[j..n] are locked.
    //
    // i and j range from `0..=n`.
    //
    // Base case: 0s on the diagonal (when i == j).
    let mut dp = vec![vec![0; n + 1]; 2];

    // Fill up rows from bottom to top.
    for i in (0..=n).rev() {
        for j in i + 1..=n {
            // Recursive case: we can either
            // * cash-in the current number, or
            // * unlock that many more numbers.
            let cash = nums[i] + dp[(i + 1) % 2][j];
            let unlock = dp[(i + 1) % 2][(j + nums[i] as usize).clamp(0, n)];
            dp[i % 2][j] = max(cash, unlock);
        }
    }

    dp[0][1]
}

fn _original_soln(nums: &[u32]) -> u32 {
    let n = nums.len();
    assert_ne!(n, 0);

    let mut dp = vec![vec![u32::MAX; n + 1]; n + 1];

    // Base case: 0s on the diagonal.
    for i in 0..=n {
        dp[i][i] = 0;
    }

    for i in (0..=n).rev() {
        for j in i + 1..=n {
            let cash = nums[i] + dp[i + 1][j];
            let unlock = dp[i + 1][(j + nums[i] as usize).clamp(0, n)];
            dp[i][j] = max(cash, unlock);
        }
    }

    for i in 0..=n {
        for j in 0..i {
            debug_assert_eq!(dp[i][j], u32::MAX);
        }
    }

    dp[0][1]
}
