use std::{error::Error, io, result::Result as StdResult};

type Result<T> = StdResult<T, Box<dyn Error>>;

fn main() -> Result<()> {
    let mut lines = io::stdin().lines();
    let _num_tests: usize = lines.next().ok_or("no input")??.parse()?;

    while let Some(line) = lines.next() {
        let n = line?.parse()?;
        let x: Vec<_> = lines
            .next()
            .ok_or("expected nums")??
            .split_whitespace()
            .map(str::parse)
            .collect::<StdResult<_, _>>()?;
        debug_assert_eq!(x.len(), n);

        let ans = soln(&x);
        for (i, a) in ans.into_iter().enumerate() {
            if i != 0 {
                print!(" ");
            }
            print!("{a}");
        }
        println!();
    }

    Ok(())
}

fn soln(x: &[u64]) -> Vec<u64> {
    let n = x.len();
    assert!(n >= 1);

    // Sort x's, but remember the original order.
    let mut tagged: Vec<_> = x.iter().copied().enumerate().collect();
    tagged.sort_unstable_by_key(|&(_orig_idx, x)| x);
    let (orig_idxs, x): (Vec<_>, Vec<_>) = tagged.into_iter().unzip();

    let mut left = vec![0; n];
    let mut area = 0;
    left[0] = area;
    for i in 1..n {
        let delta = x[i] - x[i - 1];
        area += delta * i as u64;
        left[i] = area;
    }

    let mut right = vec![0; n];
    let mut area = 0;
    right[n - 1] = area;
    for i in (0..n - 1).rev() {
        let delta = x[i + 1] - x[i];
        area += delta * (n - 1 - i) as u64;
        right[i] = area;
    }
    // dbg!(&right);

    let mut out = vec![0; n];
    for i in 0..n {
        // +n since areas/intervals include both endpoints.
        out[i] = left[i] + right[i] + n as u64;
    }

    // Unsort x's.
    let mut out_reordered = vec![u64::MAX; n];
    for i in 0..n {
        let orig_idx = orig_idxs[i];
        out_reordered[orig_idx] = out[i];
    }
    debug_assert!(out_reordered.iter().all(|&val| val != u64::MAX));
    out_reordered
}
