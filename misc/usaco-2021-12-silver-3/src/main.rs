use std::{error::Error, io};

fn main() -> Result<(), Box<dyn Error>> {
    let mut lines = io::stdin().lines();
    let nm: Vec<_> = lines
        .next()
        .ok_or("no input")??
        .split_whitespace()
        .map(str::parse)
        .collect::<Result<_, _>>()?;
    let &[n, m] = nm.as_slice() else {
        Err("expected 2 nums: n and m")?
    };

    let intervals: Vec<_> = lines.map(parse_line).collect::<Result<_, _>>()?;
    debug_assert_eq!(intervals.len(), n);

    let ans = soln(&intervals, m);
    for freq in ans {
        println!("{freq}");
    }

    Ok(())
}

fn parse_line(line: io::Result<String>) -> Result<(usize, usize), Box<dyn Error>> {
    let ij: Vec<_> = line?
        .split_whitespace()
        .map(str::parse)
        .collect::<Result<_, _>>()?;
    let &[i, j] = ij.as_slice() else {
        Err("expected 2 nums: i and j")?
    };
    Ok((i, j))
}

fn soln(intervals: &[(usize, usize)], m: usize) -> Vec<usize> {
    // freq maps
    let mut starts = vec![0; m + 1];
    let mut ends = vec![0; m + 1];
    for &(i, j) in intervals {
        starts[i] += 1;
        ends[j] += 1;
    }

    // freq maps
    let mut combined_starts = vec![0; 2 * m + 1];
    let mut combined_ends = vec![0; 2 * m + 1];
    for i in 0..=m {
        for j in 0..=m {
            combined_starts[i + j] += starts[i] * starts[j];
            combined_ends[i + j] += ends[i] * ends[j];
        }
    }

    let mut ans = vec![0; 2 * m + 1];
    let mut thickness = 0;
    for i in 0..=2 * m {
        thickness += combined_starts[i];
        ans[i] = thickness;
        thickness -= combined_ends[i];
    }
    ans
}
