use std::{error::Error, io};

type Result<T> = std::result::Result<T, Box<dyn Error>>;

fn main() -> Result<()> {
    let mut lines = io::stdin().lines();
    let t: usize = lines.next().ok_or("t")??.parse()?;

    for _ in 0..t {
        let l = lines.next().ok_or("l")??;
        let (a, b) = parse_line(&l)?;
        let ans = soln(a, b);
        println!("{ans}");
    }

    if lines.next().is_some() {
        return Err("expected EOF".into());
    }

    Ok(())
}

fn parse_line(l: &str) -> Result<(i32, i32)> {
    let nums: Vec<i32> = l
        .split_whitespace()
        .map(str::parse)
        .collect::<std::result::Result<_, _>>()?;
    let &[a, b] = nums.as_slice() else {
        return Err(format!("expected 2 nums, got: {}", nums.len()).into());
    };
    Ok((a, b))
}

fn soln(a: i32, b: i32) -> i32 {
    b - a
}
