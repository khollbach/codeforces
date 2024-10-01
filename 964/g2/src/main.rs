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
        dbg!(low, high);
        let (x, y) = partition_in_thirds(low, high);

        let (x2, y2) = area_query(x, y)?;

        if y2 == y {
            low = y + 1;
        } else if x2 == x {
            low = x + 1;
            high = y;
        } else {
            high = x;
        }
    }
    debug_assert_eq!(low, high);

    Ok(low)
}

/// Partition the range low..high into thirds.
fn partition_in_thirds(low: u32, high: u32) -> (u32, u32) {
    assert!(low <= high);

    // Calculate, assuming low == 0.
    let n = high - low;
    let x = n / 3;
    let y = n * 2 / 3;

    // Shift back to low == low.
    (x + low, y + low)
}

/// How long are x and y, according to the ruler?
fn area_query(x: u32, y: u32) -> Result<(u32, u32)> {
    println!("? {x} {y}");
    let line = io::stdin().lines().next().ok_or("unexpected EOF")??;
    let area: u32 = line.parse()?;

    if area == (x + 1) * (y + 1) {
        Ok((x + 1, y + 1))
    } else if area == x * (y + 1) {
        Ok((x, y + 1))
    } else if area == (x + 1) * y {
        Ok((x + 1, y))
    } else if area == x * y {
        Ok((x, y))
    } else {
        Err(format!("unexpected area for {x}, {y}: {area}").into())
    }
}
