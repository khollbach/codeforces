use std::{error::Error, io, iter::zip, process::exit, result::Result as StdResult};

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

        drop(lines); // release the lock on stdin
        soln(&nums)?;
        lines = io::stdin().lines();
    }

    Ok(())
}

fn soln(nums: &[u32]) -> Result<()> {
    println!("{}", first_missing(nums));

    let mut lines = io::stdin().lines();
    while let Some(line) = lines.next() {
        match line?.parse()? {
            y if y >= 0 => println!("{y}"),
            -1 => return Ok(()),
            -2 => exit(1),
            code => panic!("invalid negative code: {code}"),
        }
    }
    panic!("unexpected EOF");
}

fn first_missing(nums: &[u32]) -> u32 {
    for (i, &x) in zip(0.., nums) {
        if x != i {
            return i;
        }
    }
    nums.len() as u32
}
