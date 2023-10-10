use std::{error::Error, io, result::Result as StdResult};

type Result<T> = StdResult<T, Box<dyn Error>>;

fn main() -> Result<()> {
    let mut lines = io::stdin().lines();
    let num_tests: usize = lines.next().ok_or("no input")??.parse()?;

    for t in 1..=num_tests {
        let line = lines.next().ok_or("line")?;
        let n = line?.parse()?;
        let xs: Vec<_> = lines
            .next()
            .ok_or("xs")??
            .split_whitespace()
            .map(|x| x.parse().expect("x"))
            .collect();
        assert_eq!(xs.len(), n);

        let ans = soln(xs);
        println!("Case #{t}: {ans}");
    }

    Ok(())
}

fn soln(mut xs: Vec<u32>) -> f64 {
    let n = xs.len();
    if n == 5 {
        return soln_5(xs);
    }

    xs.sort_unstable();
    let low = (xs[0] as f64 + xs[1] as f64) / 2.;
    let high = (xs[n - 2] as f64 + xs[n - 1] as f64) / 2.;
    high - low
}

fn soln_5(mut xs: Vec<u32>) -> f64 {
    assert_eq!(xs.len(), 5);
    xs.sort_unstable();

    let mut ys = xs.clone();
    ys.remove(1);

    let mut zs = xs.clone();
    zs.remove(3);

    f64::max(soln(ys), soln(zs))
}
