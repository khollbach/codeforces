use std::{error::Error, io, result::Result as StdResult};

type Result<T> = StdResult<T, Box<dyn Error>>;

fn main() -> Result<()> {
    let mut lines = io::stdin().lines();
    let _num_tests: usize = lines.next().ok_or("no input")??.parse()?;

    while let Some(line) = lines.next() {
        let n = line?.parse()?;

        let grid: Vec<_> = lines
            .by_ref()
            .take(n)
            .map(|line| parse_row(line, n))
            .collect::<Result<_>>()?;
        assert_eq!(grid.len(), n);

        let ans = soln(grid);
        println!("{ans}");
    }

    Ok(())
}

fn parse_row(line: io::Result<String>, n: usize) -> Result<Vec<bool>> {
    let row: Vec<_> = line?
        .chars()
        .map(|c| match c {
            '0' => Ok(false),
            '1' => Ok(true),
            _ => Err("not a bit: {c:?}")?,
        })
        .collect::<Result<_>>()?;
    assert_eq!(row.len(), n);
    Ok(row)
}

fn soln(mut grid: Vec<Vec<bool>>) -> usize {
    // Non-empty square?
    let n = grid.len();
    for row in &grid {
        assert_eq!(row.len(), n);
    }
    if grid.is_empty() {
        return 0;
    }

    // Pad the grid, to make bounds checks go away.
    // Note that we'll be using 1-based indexing from now on.
    for row in &mut grid {
        row.insert(0, false);
        row.push(false);
    }
    grid.insert(0, vec![false; n + 2]);
    grid.push(vec![false; n + 2]);

    let mut infected = vec![vec![0u8; n + 2]; n + 2];
    let mut num_manual_flips = 0;

    for i in 1..=n {
        for j in 1..=n {
            // Automatic flips, due to infection from above.
            grid[i][j] ^= infected[i][j].count_ones() % 2 == 1;

            // Manual flip, if needed.
            if grid[i][j] {
                grid[i][j] = false;
                infected[i][j] ^= LEFT | DOWN | RIGHT;
                num_manual_flips += 1;
            }

            // Spread infection downwards.
            if infected[i][j] & LEFT != 0 {
                infected[i + 1][j - 1] ^= LEFT;
            }
            if infected[i][j] & DOWN != 0 {
                infected[i + 1][j] ^= LEFT | DOWN | RIGHT;
            }
            if infected[i][j] & RIGHT != 0 {
                infected[i + 1][j + 1] ^= RIGHT;
            }
        }
    }

    num_manual_flips
}

// Bitflags; representing any subset of {left, down, right}.
//type Infection = u8;
const LEFT: u8 = 0b_100;
const DOWN: u8 = 0b_010;
const RIGHT: u8 = 0b_001;

// Turns out this doesn't work.
//
// E.g. consider the case of just a single cone (desired answer = 1).
// It works at levels 0 and 1: e.g. you spread the flip left, right, and down.
// But then at level 2, you get those three flips above interacting in ways that
// aren't correct (e.g. cancelling each other out when they shouldn't.)
fn _attempt_1(mut grid: Vec<Vec<bool>>) -> usize {
    // Non-empty square?
    let n = grid.len();
    for row in &grid {
        assert_eq!(row.len(), n);
    }
    if grid.is_empty() {
        return 0;
    }

    // We only store the parity: `num_flips[i][j]` is true iff the number of
    // flips is odd.
    let mut num_flips = vec![vec![false; n]; n];
    let mut num_manual_flips = 0;

    for i in 0..n {
        for j in 0..n {
            if i != 0 {
                // How many flips from above?
                if j != 0 {
                    num_flips[i][j] ^= num_flips[i - 1][j - 1];
                }
                num_flips[i][j] ^= num_flips[i - 1][j];
                if j != n - 1 {
                    num_flips[i][j] ^= num_flips[i - 1][j + 1];
                }

                // Apply the flips from above.
                grid[i][j] ^= num_flips[i][j];
            }

            // Perform a manual flip, if needed.
            if grid[i][j] {
                grid[i][j] = false;
                num_flips[i][j] ^= true;
                num_manual_flips += 1;
            }
        }
    }

    num_manual_flips
}
