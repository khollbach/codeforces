use std::{error::Error, io};

type Result<T> = std::result::Result<T, Box<dyn Error>>;

fn main() -> Result<()> {
    let mut lines = io::stdin().lines();
    let t: usize = lines.next().ok_or("t")??.parse()?;
    drop(lines);

    for _ in 1..=t {
        let ans = find_missing_number()?;
        println!("! {ans}");
    }

    if io::stdin().lines().next().is_some() {
        return Err("expected EOF".into());
    }

    Ok(())
}

fn find_missing_number() -> Result<u32> {
    // Find the smallest number that measures incorrectly,
    // according to the ruler.

    let mut low = 1;
    let mut high = 1000; // exclusive

    while low < high {
        let mid = (low + high) / 2;

        if length_query(mid)? == mid {
            low = mid + 1;
        } else {
            high = mid;
        }
    }
    debug_assert_eq!(low, high);

    Ok(low)
}

/// How long is x, according to the ruler?
fn length_query(x: u32) -> Result<u32> {
    println!("? {x} 1");
    let line = io::stdin().lines().next().ok_or("unexpected EOF")??;
    let len = line.parse()?;
    Ok(len)
}
