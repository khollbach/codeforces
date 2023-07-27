use std::{collections::HashSet, error::Error, io, result::Result as StdResult};

type Result<T> = StdResult<T, Box<dyn Error>>;

fn main() -> Result<()> {
    let mut lines = io::stdin().lines();
    let _num_tests: usize = lines.next().ok_or("no input")??.parse()?;

    while let Some(line) = lines.next() {
        let nm: Vec<_> = line?
            .split_whitespace()
            .map(str::parse)
            .collect::<StdResult<_, _>>()?;
        let &[n, m] = nm.as_slice() else {
            Err("expected 2 nums: n and m")?
        };

        let s: Vec<_> = lines
            .next()
            .ok_or("expected line with binary string")??
            .chars()
            .map(Bit::new)
            .collect::<Result<_>>()?;
        debug_assert_eq!(s.len(), n);

        let ranges: Vec<_> = lines
            .by_ref()
            .take(m)
            .map(|line| parse_range(&line?, n))
            .collect::<Result<_>>()?;
        debug_assert_eq!(ranges.len(), m);

        let ans = soln(s, ranges);
        println!("{ans}");
    }

    Ok(())
}

fn parse_range(line: &str, n: usize) -> Result<(usize, usize)> {
    let lr: Vec<_> = line
        .split_whitespace()
        .map(str::parse)
        .collect::<StdResult<_, _>>()?;
    let &[l, r] = lr.as_slice() else {
        Err("expected 2 nums: l and r")?
    };

    let valid = l != 0 && l <= r && r <= n;
    if !valid {
        Err(format!("invalid range: [{l}, {r}] vs [1, {n}]"))?
    }

    // 0-indexed, left-inclusive, if you please.
    Ok((l - 1, r))
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Bit {
    Zero,
    One,
}

impl Bit {
    fn new(c: char) -> Result<Self> {
        let bit = match c {
            '0' => Self::Zero,
            '1' => Self::One,
            _ => Err(format!("expected '0' or '1', got {c:?}"))?,
        };
        Ok(bit)
    }
}

fn soln(mut s: Vec<Bit>, mut ranges: Vec<(usize, usize)>) -> usize {
    // Ensure `s` starts with 0 and ends with 1.
    s.insert(0, Bit::Zero);
    s.push(Bit::One);

    // Adjust indices to keep up with the above transformation.
    for (i, j) in &mut ranges {
        *i += 1;
        *j += 1;
    }

    // The blocks of s are the maximal substrings of all 0s or all 1s.
    //
    // `block_starts[i]` is the starting index of i's block.
    // `block_ends[i]` is the ending index, exclusive.
    let mut block_starts = vec![usize::MAX; s.len()];
    let mut block_ends = vec![usize::MAX; s.len()];

    let mut start = 0;
    loop {
        // Find the start of the next block.
        let end = (start..s.len())
            .find(|&i| s[i] != s[start])
            .unwrap_or(s.len());

        for i in start..end {
            block_starts[i] = start;
            block_ends[i] = end;
        }

        if end == s.len() {
            break;
        } else {
            start = end;
        }
    }

    debug_assert!(block_starts.iter().all(|&idx| idx != usize::MAX));
    debug_assert!(block_ends.iter().all(|&idx| idx != usize::MAX));

    // "Expand" each range to include 0s to its left and 1s to its right.
    // Then count the number of unique ranges.
    ranges
        .into_iter()
        .map(|(i, j)| {
            let start = if s[i - 1] == Bit::Zero {
                block_starts[i - 1]
            } else {
                i
            };

            let end = if s[j] == Bit::One { block_ends[j] } else { j };

            // !!! Edge-case: need to detect if sorting this range *does nothing*.
            // todo

            (start, end)
        })
        .collect::<HashSet<_>>()
        .len()
}
