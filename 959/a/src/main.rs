use std::io;

fn main() {
    let mut lines = io::stdin().lines();

    let _num_tests: usize = lines.next().unwrap().unwrap().parse().unwrap();

    while let Some(l) = lines.next() {
        let dims: Vec<usize> = l
            .unwrap()
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        let &[n_rows, n_cols] = dims.as_slice() else {
            panic!()
        };

        let mut rows = Vec::with_capacity(n_rows);
        for l in lines.by_ref().take(n_rows) {
            let row: Vec<u32> = l
                .unwrap()
                .split_whitespace()
                .map(|s| s.parse().unwrap())
                .collect();
            assert_eq!(row.len(), n_cols);
            rows.push(row);
        }

        // Edge-case: 1x1.
        if (n_rows, n_cols) == (1, 1) {
            println!("-1");
            continue;
        }

        let ans = soln(rows);
        for row in ans {
            for x in row {
                print!("{x} ");
            }
            println!();
        }
    }
}

fn soln(mut m: Vec<Vec<u32>>) -> Vec<Vec<u32>> {
    for row in &mut m {
        row.rotate_right(1);
    }

    let n_rows = m.len();
    let n_cols = m[0].len();
    for j in 0..n_cols {
        // Rotate column j upwards by 1.
        let top = m[0][j];
        for i in 0..n_rows - 1 {
            m[i][j] = m[i + 1][j];
        }
        m[n_rows - 1][j] = top;
    }

    m
}
