use std::{error::Error, io, iter::zip, result::Result as StdResult};

type Result<T> = StdResult<T, Box<dyn Error>>;

fn main() -> Result<()> {
    let mut lines = io::stdin().lines();
    let _num_tests: usize = lines.next().ok_or("no input")??.parse()?;

    while let Some(line) = lines.next() {
        let n = line?.parse()?;
        let nums: Vec<_> = lines
            .next()
            .ok_or("expected nums")??
            .split_whitespace()
            .map(str::parse)
            .collect::<StdResult<_, _>>()?;
        assert_eq!(nums.len(), n);

        let ans = soln(&nums);
        for x in ans {
            print!("{x} ");
        }
        println!();
    }

    Ok(())
}

fn soln(nums: &[u32]) -> Vec<u32> {
    let n = nums.len();
    let mut tagged: Vec<_> = nums.iter().enumerate().collect();
    tagged.sort_by_key(|&(_idx, x)| x);

    let mut out = vec![0; n];
    for ((i, _), y) in zip(tagged, (1..=n as u32).rev()) {
        out[i] = y;
    }
    out
}
