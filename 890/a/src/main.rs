use std::{error::Error, io, result::Result as StdResult};

type Result<T> = StdResult<T, Box<dyn Error>>;

fn main() -> Result<()> {
    let mut lines = io::stdin().lines();
    let _num_tests: usize = lines.next().ok_or("no input")??.parse()?;

    while let Some(line) = lines.next() {
        let n = line?.parse()?;

        let nums: Vec<_> = lines
            .next()
            .ok_or("expected line of nums")??
            .split_whitespace()
            .map(str::parse)
            .collect::<StdResult<_, _>>()?;
        debug_assert_eq!(nums.len(), n);

        let ans = soln(&nums);
        println!("{ans}");
    }

    Ok(())
}

fn soln(nums: &[i32]) -> usize {
    debug_assert!(nums.iter().all(|&x| x > 0));

    nums.windows(2)
        .flat_map(|pair| {
            let d = pair[1] - pair[0];
            if d < 0 {
                Some(pair[0] as usize)
            } else {
                None
            }
        })
        .max()
        .unwrap_or(0)
}
