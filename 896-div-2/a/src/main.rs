use std::{error::Error, io, result::Result as StdResult};

type Result<T> = StdResult<T, Box<dyn Error>>;

fn main() -> Result<()> {
    let mut lines = io::stdin().lines();
    let _num_tests: usize = lines.next().ok_or("no input")??.parse()?;

    while let Some(line) = lines.next() {
        let n = line?.parse()?;
        let nums: Vec<u8> = lines.next().ok_or("expected nums")??.split_whitespace().map(str::parse).collect::<StdResult<_, _>>()?;
        assert_eq!(nums.len(), n);

        let ans = soln(n as u32);
        println!("{}", ans.len());
        for (i, j) in ans {
            println!("{i} {j}");
        }
    }

    Ok(())
}

fn soln(n: u32) -> Vec<(u32, u32)> {
    vec![(1, n), (1, n), (1, n-1), (1, n-1), (2, n), (2, n)]
}
