use std::{
    cmp::min,
    collections::{HashSet, VecDeque},
    error::Error,
    io,
    result::Result as StdResult,
};

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
            Err("expected 2 nums")?
        };

        let matrix = soln(n, m);
        println!("{}", beauty(&matrix));
        for row in matrix {
            for x in row {
                print!("{x} ");
            }
            println!();
        }
    }

    Ok(())
}

type Matrix = Vec<Vec<u32>>;

fn soln(n: usize, m: usize) -> Matrix {
    assert_ne!(n, 0);
    assert_ne!(m, 0);

    let mut perm: VecDeque<_> = (0..m as u32).collect();

    // achieve maximum beauty
    let mut matrix = vec![];
    for _ in 0..min(n, m - 1) {
        perm.rotate_right(1); // rotate first, to skip identity perm
        matrix.push(perm.iter().copied().collect());
    }

    // pad with duplicate rows
    while matrix.len() < n {
        matrix.push(perm.iter().copied().collect());
    }

    matrix
}

fn beauty(matrix: &Matrix) -> u32 {
    let n = matrix.len();
    let m = matrix[0].len();

    let mut column_vals = HashSet::new();
    for j in 0..m {
        // let mut sum = 0;
        // for i in 0..n {
        //     sum += matrix[i][j];
        // }
        // column_vals.insert(sum);

        let mut uniq = HashSet::new();
        for i in 0..n {
            uniq.insert(matrix[i][j]);
        }
        for x in 0.. {
            if !uniq.contains(&x) {
                column_vals.insert(x);
                break;
            }
        }
    }

    for x in 0.. {
        if !column_vals.contains(&x) {
            return x;
        }
    }
    unreachable!();
}
