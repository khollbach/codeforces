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
            Err("expected two numbers: n and k")?
        };

        let colors: Vec<_> = lines
            .next()
            .ok_or("expected line of colors")??
            .split_whitespace()
            .map(str::parse)
            .collect::<StdResult<_, _>>()?;
        debug_assert_eq!(colors.len(), n);

        let ans = if can_traverse(&colors, k) {
            "YES"
        } else {
            "NO"
        };
        println!("{ans}");
    }

    Ok(())
}

fn can_traverse(colors: &[u32], block_len: usize) -> bool {
    let Some(first_block_end) = earliest_first_block_end(colors, block_len) else {
        return false;
    };

    // edge case: path of exactly 1 block
    let start_color = colors[0];
    let end_color = *colors.last().unwrap();
    if start_color == end_color {
        return true;
    }

    let Some(last_block_start) = latest_last_block_start(colors, block_len) else {
        return false;
    };

    // we'll never need more than 2 blocks
    // so as long as we can jump from the first block to the second, we're good.
    first_block_end < last_block_start
}

fn earliest_first_block_end(colors: &[u32], block_len: usize) -> Option<usize> {
    let start_color = colors[0];
    assert_ne!(block_len, 0);

    let mut count = 0;

    for (i, &c) in colors.iter().enumerate() {
        if c == start_color {
            count += 1;
        }

        if count == block_len {
            return Some(i);
        }
    }

    None
}

fn latest_last_block_start(colors: &[u32], block_len: usize) -> Option<usize> {
    let end_color = *colors.last().unwrap();
    assert_ne!(block_len, 0);

    let mut count = 0;

    for (i, &c) in colors.iter().enumerate().rev() {
        if c == end_color {
            count += 1;
        }

        if count == block_len {
            return Some(i);
        }
    }

    None
}
