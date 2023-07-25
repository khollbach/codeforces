use std::{error::Error, io, result::Result as StdResult};

type Result<T> = StdResult<T, Box<dyn Error>>;

fn main() -> Result<()> {
    let mut lines = io::stdin().lines();
    let _num_tests: usize = lines.next().ok_or("no input")??.parse()?;

    while let Some(line) = lines.next() {
        let nmkv: Vec<_> = line?
            .split_whitespace()
            .map(str::parse)
            .collect::<StdResult<_, _>>()?;
        let &[n, m, k, v] = nmkv.as_slice() else {
            Err("expected 4 numbers: n, m, k, v")?
        };

        let heights: Vec<_> = lines
            .next()
            .ok_or("expected line of heights")??
            .split_whitespace()
            .map(str::parse)
            .collect::<StdResult<_, _>>()?;
        debug_assert_eq!(heights.len(), n as usize);

        let ans = how_many_conversations(m, k, v, &heights);
        println!("{ans}");
    }

    Ok(())
}

fn how_many_conversations(num_steps: u32, step_size: u32, vlad: u32, people: &[u32]) -> usize {
    let max_diff = (num_steps - 1) * step_size;

    people
        .iter()
        .filter(|&&h| {
            let diff = vlad.abs_diff(h);
            diff != 0 && diff <= max_diff && diff % step_size == 0
        })
        .count()
}
