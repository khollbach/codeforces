use std::{cmp::min, error::Error, io};

type Result<T> = std::result::Result<T, Box<dyn Error>>;

fn main() -> Result<()> {
    let mut lines = io::stdin().lines();
    let t: usize = lines.next().ok_or("t")??.parse()?;

    for _ in 0..t {
        let l = lines.next().ok_or("l")??;
        let nums: Vec<usize> = l
            .split_whitespace()
            .map(str::parse)
            .collect::<std::result::Result<_, _>>()?;
        let &[n, row1, col1, row2, col2] = nums.as_slice() else {
            panic!("{}", nums.len());
        };

        // 0-based indexing, please!
        let p1 = Point {
            row: row1 - 1,
            col: col1 - 1,
        };
        let p2 = Point {
            row: row2 - 1,
            col: col2 - 1,
        };

        let ans = min_energy_to_move(n, p1, p2);
        println!("{ans}");
    }

    assert!(lines.next().is_none());

    Ok(())
}

#[derive(Debug, Clone, Copy)]
struct Point {
    row: usize,
    col: usize,
}

impl Point {
    /// "Fold" the matrix in half twice, so that everything
    /// ends up in the top-left quadrant.
    fn fold(self, n: usize) -> Self {
        Self {
            row: fold(self.row, n),
            col: fold(self.col, n),
        }
    }
}

/// "Fold" the range 0..n in half, returning the new value of i.
fn fold(i: usize, n: usize) -> usize {
    assert_eq!(n % 2, 0);
    assert!(i < n, "{i} {n}");
    if i < n / 2 {
        i
    } else {
        n - 1 - i
    }
}

fn min_energy_to_move(n: usize, start: Point, end: Point) -> usize {
    assert_eq!(n % 2, 0);

    let start = start.fold(n);
    let end = end.fold(n);

    let start_belt = min(start.row, start.col);
    let end_belt = min(end.row, end.col);
    let energy_cost = start_belt.abs_diff(end_belt);
    energy_cost
}
