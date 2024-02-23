use std::io;

fn main() {
    let mut lines = io::stdin().lines().skip(1).map(Result::unwrap);
    while let Some(nk) = lines.next() {
        let nk: Vec<u32> = nk.split_whitespace().map(|s| s.parse().unwrap()).collect();
        let &[n, k] = &nk[..] else { panic!() };
        let ans = soln(n + 1, k - 1) + 1;
        println!("{ans}");
    }
}

/*

n = 10^9  -- O(n) is borderline
*but* on top, we have 5*10^4 test cases -- so O(n) infeasable

aiming for O~(1)

n=64

n=20

1 3 5  7  9  11 13 15 17 19
2 6 10 14 18 22 26 30 34 38     -- 2^1 * odd-stuff
--                              -- 3 * odd-stuff  -- still odd
4 12 20 28 36 44 52 60 68 76    -- 2^2 * odd-stuff
--                              -- 5 7 etc ... (skipped)
8 24 40 56 72 88 104 120        -- 8=2^3 * odd-stuff
16 ...
32 ...
64
...



*/


/// n is exclusive
/// k is zero-based
/// answer is zero-based
fn soln(n: u32, k: u32) -> u32 {
    assert!(k < n);
    if k < n / 2 {
        return k * 2;
    }

    (soln(n / 2, k - n / 2) << 1) + 1
}
