use std::{error::Error, io};

type Result<T> = std::result::Result<T, Box<dyn Error>>;

fn main() -> Result<()> {
    let mut lines = io::stdin().lines();
    let t: usize = lines.next().ok_or("t")??.parse()?;

    for _ in 0..t {
        let l = lines.next().ok_or("l")??;
        let n: usize = l.parse()?;

        let mut indices = Vec::with_capacity(n);
        for l in lines.by_ref().take(n) {
            for (i, c) in l?.chars().enumerate() {
                match c {
                    '.' => (),
                    '#' => indices.push(i),
                    _ => return Err(format!("unexpected char: {c}").into()),
                }
            }
        }

        let mut first = true;
        for idx in indices.iter().rev() {
            if first {
                first = false;
            } else {
                print!(" ");
            }
            print!("{}", idx + 1);
        }
        println!();
    }

    if lines.next().is_some() {
        return Err("expected EOF".into());
    }

    Ok(())
}
