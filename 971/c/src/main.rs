use std::{error::Error, io};

type Result<T> = std::result::Result<T, Box<dyn Error>>;

fn main() -> Result<()> {
    let mut lines = io::stdin().lines();
    let t: usize = lines.next().ok_or("t")??.parse()?;

    for _ in 0..t {
        let l = lines.next().ok_or("l")??;
        let (x, y, k) = parse_line(&l)?;
        let ans = min_hops(x, y, k);
        println!("{ans}");
    }

    if lines.next().is_some() {
        return Err("expected EOF".into());
    }

    Ok(())
}

fn parse_line(l: &str) -> Result<(u32, u32, u32)> {
    let nums: Vec<u32> = l
        .split_whitespace()
        .map(str::parse)
        .collect::<std::result::Result<_, _>>()?;
    let &[x, y, k] = nums.as_slice() else {
        return Err(format!("expected 3 nums, got: {}", nums.len()).into());
    };
    Ok((x, y, k))
}

fn min_hops(x: u32, y: u32, k: u32) -> u32 {
    let x_hops = ceil_div(x, k);
    let y_hops = ceil_div(y, k);

    // BUG: had this flipped
    if y_hops >= x_hops {
        y_hops * 2
    } else {
        x_hops * 2 - 1
    }
}

fn ceil_div(a: u32, b: u32) -> u32 {
    assert_ne!(b, 0);
    let extra = (a % b != 0) as u32;
    a / b + extra
}
