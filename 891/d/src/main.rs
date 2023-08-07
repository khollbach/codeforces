use std::{error::Error, io, iter::zip, result::Result as StdResult};

type Result<T> = StdResult<T, Box<dyn Error>>;

fn main() -> Result<()> {
    let mut lines = io::stdin().lines();
    let _num_tests: usize = lines.next().ok_or("no input")??.parse()?;

    while let Some(line) = lines.next() {
        let n = line?.parse()?;

        let a: Vec<_> = lines
            .next()
            .ok_or("expected list a")??
            .split_whitespace()
            .map(str::parse)
            .collect::<StdResult<_, _>>()?;
        let b: Vec<_> = lines
            .next()
            .ok_or("expected list b")??
            .split_whitespace()
            .map(str::parse)
            .collect::<StdResult<_, _>>()?;
        debug_assert_eq!(a.len(), n);
        debug_assert_eq!(b.len(), n);

        let ans = soln(&a, &b);

        println!("{}", ans.len());
        for (i, idx) in ans.into_iter().enumerate() {
            if i != 0 {
                print!(" ");
            }
            print!("{}", idx + 1);
        }
        println!();
    }

    Ok(())
}

fn soln(a: &[i32], b: &[i32]) -> Vec<usize> {
    assert!(a.len() >= 2);
    assert_eq!(a.len(), b.len());
    let delta: Vec<_> = zip(a, b).map(|(x, y)| x - y).collect();

    let max = *delta.iter().max().unwrap();

    delta
        .into_iter()
        .enumerate()
        .filter_map(|(i, d)| if d == max { Some(i) } else { None })
        .collect()
}
