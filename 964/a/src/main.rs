use std::{error::Error, io};

type Result<T> = std::result::Result<T, Box<dyn Error>>;

fn main() -> Result<()> {
    let mut lines = io::stdin().lines();
    let t: usize = lines.next().ok_or("t")??.parse()?;

    for _ in 1..=t {
        let n: u32 = lines.next().ok_or("n")??.parse()?;
        if !(10 <= n && n <= 99) {
            return Err(format!("{n}").into());
        }

        let ans = digit_sum(n);
        println!("{ans}");
    }

    if lines.next().is_some() {
        return Err("expected EOF".into());
    }

    Ok(())
}

fn digit_sum(n: u32) -> u32 {
    n.to_string().chars().map(|c| c.to_digit(10).unwrap()).sum()
}
