use std::{error::Error, io, result::Result as StdResult};

type Result<T> = StdResult<T, Box<dyn Error>>;

fn main() -> Result<()> {
    let mut lines = io::stdin().lines();
    let _num_tests: usize = lines.next().ok_or("no input")??.parse()?;

    while let Some(line) = lines.next() {
        let line = line?;
        let nc: Vec<_> = line.split_whitespace().collect();
        let &[n, c] = nc.as_slice() else {
            Err("expected two words: n and c")?
        };

        let s: Vec<_> = lines
            .next()
            .ok_or("expected line of side lengths")??
            .split_whitespace()
            .map(str::parse)
            .collect::<StdResult<_, _>>()?;
        if s.len() != n.parse()? {
            Err("expected n side lengths")?
        }

        let ans = solve_for_w(&s, c.parse()?)?;
        println!("{ans}");
    }

    Ok(())
}

fn solve_for_w(s: &[u64], c: u64) -> Result<u64> {
    let a = s.len() as i128;
    let b = 2 * s.iter().sum::<u64>() as i128;
    let c = s.iter().map(|si| si.pow(2)).sum::<u64>() as i128 - c as i128;

    // Solve for `a * x^2 + b * x + c == 0`, using the formula:
    // (-b + sqrt(b^2 - 4ac)) / 2a

    let d = b.pow(2) - 4 * a * c; // This overflowed when we used i64's.
    if d < 0 {
        Err(format!("d negative: {d}"))?
    }
    let sqrt_d = (d as f64).sqrt(); // TODO: this cast seems dangerous
    if sqrt_d.fract() != 0. {
        Err(format!("sqrt(d) fractional: {sqrt_d}"))?
    }
    let sqrt_d = sqrt_d.trunc() as i128;

    let top = -b + sqrt_d;
    let bot = 2 * a;
    if top % bot != 0 {
        Err(format!("-b+sqrt(d) not divisible by 2a: {top} / {bot}"))?
    }
    let x = top / bot;
    if x < 0 {
        Err(format!("x negative: {x}"))?
    }

    // Above we solved for 2w, the total side length to add to each square.
    // Divide by 2 to get just w, the width to frame each square.
    if x % 2 != 0 {
        Err(format!("2*w odd: {x}"))?
    }
    let w = x / 2;
    if w > u64::MAX as i128 {
        Err(format!("w overflows u64: {w}"))?
    }

    Ok(w as u64)
}
