use std::{error::Error, io, result::Result as StdResult};

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
            Err("expected 2 nums: n m")?
        };

        let grid: Vec<String> = lines.by_ref().take(n).collect::<StdResult<_, _>>()?;
        assert_eq!(grid.len(), n);
        for s in &grid {
            assert_eq!(s.len(), m);
        }

        let ans = if soln(&grid) { "YES" } else { "NO" };
        println!("{ans}");
    }

    Ok(())
}

fn soln(grid: &[String]) -> bool {
    let mut word = b"vika".iter().copied().peekable();

    let n = grid.len();
    let m = grid[0].len();

    for col in 0..m {
        for row in 0..n {
            if word.peek() == Some(&grid[row].as_bytes()[col]) {
                word.next();
                break;
            }
        }
    }

    word.peek().is_none()
}
