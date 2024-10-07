use std::{error::Error, io};

type Result<T> = std::result::Result<T, Box<dyn Error>>;

fn main() -> Result<()> {
    let mut lines = io::stdin().lines();
    let t: usize = lines.next().ok_or("t")??.parse()?;

    for _ in 0..t {
        let l = lines.next().ok_or("l")??;
        let nums: Vec<u64> = l
            .split_whitespace()
            .map(str::parse)
            .collect::<std::result::Result<_, _>>()?;
        let &[n, x, y] = nums.as_slice() else {
            panic!("{}", nums.len());
        };

        let row = x - 1;
        let col = y - 1;

        let fib = first_n_fibonacci_numbers(n + 2);
        if can_cut(&fib, n, row, col) {
            println!("YES");
        } else {
            println!("NO");
        }
    }

    assert!(lines.next().is_none());

    Ok(())
}

fn first_n_fibonacci_numbers(n: u64) -> Vec<u64> {
    let mut out = Vec::with_capacity(n as usize);

    let mut a = 1;
    let mut b = 1;

    for _ in 0..n {
        out.push(a);
        (a, b) = (b, a + b);
    }

    out
}

/// Can we cut out a 1x1 square at (row, col) and still tile with Fibonacci rectangles?
fn can_cut(fib: &[u64], n: u64, row: u64, col: u64) -> bool {
    assert!(row < fib[n as usize]);
    assert!(col < fib[n as usize + 1]);
    if n == 0 {
        debug_assert_eq!((row, col), (0, 0));
        return true;
    }

    let col = fold(Fold {
        value: col,
        len: fib[n as usize + 1],
    });

    if col >= fib[n as usize - 1] {
        return false;
    }

    // Note the swap: (col, row).
    can_cut(fib, n - 1, col, row)
}

fn fold(f: Fold) -> u64 {
    assert!(f.value < f.len);
    if f.value < f.len / 2 {
        f.value
    } else {
        f.len - 1 - f.value
    }
}

struct Fold {
    value: u64,
    len: u64,
}
