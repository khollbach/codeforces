use std::{cmp::min, error::Error, io};

type Result<T> = std::result::Result<T, Box<dyn Error>>;

fn main() -> Result<()> {
    let mut lines = io::stdin().lines();
    let t: usize = lines.next().ok_or("t")??.parse()?;

    for _ in 0..t {
        let l = lines.next().ok_or("n")??;
        let n: usize = l.parse()?;

        let l = lines.next().ok_or("b")??;
        let b: Vec<u32> = l
            .split_whitespace()
            .map(str::parse)
            .collect::<std::result::Result<_, _>>()?;
        assert_eq!(b.len(), n - 1);

        let a = restore_a(&b);
        for x in a {
            print!("{x} ");
        }
        println!();
    }

    assert!(lines.next().is_none());

    Ok(())
}

fn restore_a(b: &[u32]) -> Vec<u32> {
    let mut a: Vec<u32> = b.windows(2).map(|pair| min(pair[0], pair[1])).collect();

    a.insert(0, b[0]);
    a.push(*b.last().unwrap());

    a
}
