use std::{error::Error, io, result::Result as StdResult};

type Result<T> = StdResult<T, Box<dyn Error>>;

fn main() -> Result<()> {
    let mut lines = io::stdin().lines();
    let _num_tests: usize = lines.next().ok_or("no input")??.parse()?;

    while let Some(line) = lines.next() {
        let n = line?.parse()?;

        let traps: Vec<_> = lines.by_ref().take(n).map(parse_line).collect::<Result<_>>()?;
        assert_eq!(traps.len(), n);

        let ans = soln(&traps);
        println!("{ans}");
    }

    Ok(())
}

fn parse_line(line: io::Result<String>) -> Result<(u32, u32)> {
    let ds: Vec<_> = line?.split_whitespace().map(str::parse).collect::<StdResult<_, _>>()?;
    let &[d, s] = ds.as_slice() else {
        Err("expected 2 nums")?
    };
    Ok((d, s))
}

/// [(d, s)]
fn soln(traps: &[(u32, u32)]) -> u32 {
    traps.iter().map(|&(d, s)| d + (s - 1) / 2).min().unwrap()
}
