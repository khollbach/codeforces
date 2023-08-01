use std::{error::Error, io, result::Result as StdResult};

type Result<T> = StdResult<T, Box<dyn Error>>;

fn main() -> Result<()> {
    let mut lines = io::stdin().lines();
    let _num_tests: usize = lines.next().ok_or("no input")??.parse()?;

    while let Some(line) = lines.next() {
        let n = line?.parse()?;
        debug_assert!(n >= 2);

        let a: Vec<_> = lines
            .next()
            .ok_or("expected line of nums")??
            .split_whitespace()
            .map(str::parse)
            .collect::<StdResult<_, _>>()?;
        debug_assert_eq!(a.len(), n);

        let ans = min_edits_to_unsort(&a).expect("a must have at least two nums");

        // I mis-read the problem; each edit will actually decrease a gap by 2.
        let ans = ceil_div(ans, 2);

        println!("{ans}");
    }

    Ok(())
}

/// An edit is either an increment or a decrement of a value `a_i`.
fn min_edits_to_unsort(a: &[u32]) -> Option<u32> {
    let gaps = a.windows(2).map(|pair| pair[1] as i32 - pair[0] as i32);
    let smallest = gaps.min()?;
    let num_edits = if smallest >= 0 { smallest + 1 } else { 0 };
    Some(num_edits as u32)
}

fn ceil_div(x: u32, y: u32) -> u32 {
    let half = x as f64 / 2.;
    let rounded = half.ceil();
    rounded as u32
}
