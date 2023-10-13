use std::{cmp, error::Error, io, result::Result as StdResult};

type Result<T> = StdResult<T, Box<dyn Error>>;

fn main() -> Result<()> {
    let mut lines = io::stdin().lines();
    let _num_tests: usize = lines.next().ok_or("no input")??.parse()?;

    while let Some(line) = lines.next() {
        let n: usize = line?.parse()?;
        let mut grid = Vec::with_capacity(n);
        for _ in 0..n {
            let s = lines.next().unwrap().unwrap();
            assert_eq!(s.len(), n);
            grid.push(s.into_bytes());
        }

        let ans = soln(&grid);
        println!("{ans}");
    }

    Ok(())
}

fn soln(grid: &Vec<Vec<u8>>) -> u32 {
    let n = grid.len();
    let mut num_ops = 0;
    for mut i in 0..n / 2 {
        for mut j in 0..n / 2 {
            let mut sum = 0u32;
            let mut max = u32::MIN;
            for _ in 0..4 {
                let x = grid[i][j] as u32;
                sum += x;
                max = cmp::max(max, x);
                (i, j) = (j, n - 1 - i);
            }
            num_ops += 4 * max - sum;
        }
    }
    num_ops
}
