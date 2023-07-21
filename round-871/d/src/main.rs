use std::{error::Error, io, result::Result as StdResult};

type Result<T> = StdResult<T, Box<dyn Error>>;

fn main() -> Result<()> {
    let mut lines = io::stdin().lines();
    let _num_tests = lines.next().ok_or("no input")??;

    for line in lines {
        let nums: Vec<_> = line?
            .split_whitespace()
            .map(str::parse)
            .collect::<StdResult<_, _>>()?;
        let &[x, y] = nums.as_slice() else {
            Err("expected two numbers")?
        };

        let ans = if can_transform(x, y) { "YES" } else { "NO" };
        println!("{ans}");
    }

    Ok(())
}

fn can_transform(mut x: u32, mut y: u32) -> bool {
    let (x2, x3) = factor_out_2s_and_3s(&mut x);
    let (y2, y3) = factor_out_2s_and_3s(&mut y);

    let Some(diff3) = x3.checked_sub(y3) else {
        return false;
    };

    let Some(diff2) = y2.checked_sub(x2) else {
        return false;
    };

    // We need enough 3s in `x` to turn into 2s in `y`.
    // All other factors should just be identical.
    diff2 <= diff3 && x == y
}

fn factor_out_2s_and_3s(x: &mut u32) -> (u32, u32) {
    if *x == 0 {
        return (0, 0);
    }

    let mut num_2s = 0;
    while *x % 2 == 0 {
        *x /= 2;
        num_2s += 1;
    }

    let mut num_3s = 0;
    while *x % 3 == 0 {
        *x /= 3;
        num_3s += 1;
    }

    (num_2s, num_3s)
}
