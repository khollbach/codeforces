use std::{error::Error, io, result::Result as StdResult};

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
            Err("expected 2 nums")?
        };

        let mut s = lines.next().ok_or("expected s")??;
        assert_eq!(s.len(), n);

        soln(&mut s, k);
        println!("{s}");
    }

    Ok(())
}

fn soln(s: &mut String, k: usize) {
    assert!(s.is_ascii());

    // SAFETY: we only ever perform swaps, and the string is ascii.
    let s = unsafe { s.as_bytes_mut() };

    if k % 2 == 0 {
        s.sort_unstable();
    } else {
        let (mut evens, mut odds): (Vec<_>, _) =
            s.iter().copied().enumerate().partition(|(i, _)| i % 2 == 0);
        evens.sort_unstable_by_key(|&(_, c)| c);
        odds.sort_unstable_by_key(|&(_, c)| c);

        // interleave
        for (i, (_orig_idx, e)) in evens.into_iter().enumerate() {
            s[2 * i] = e;
        }
        for (i, (_orig_idx, o)) in odds.into_iter().enumerate() {
            s[2 * i + 1] = o;
        }
    }
}
