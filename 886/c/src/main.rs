use std::{error::Error, io, result::Result as StdResult};

type Result<T> = StdResult<T, Box<dyn Error>>;

fn main() -> Result<()> {
    let mut lines = io::stdin().lines();
    let num_tests: usize = lines.next().ok_or("no input")??.parse()?;

    for _ in 0..num_tests {
        let mut ans = String::new();

        for line in lines.by_ref().take(8) {
            for c in line?.chars() {
                if c != '.' {
                    ans.push(c);
                }
            }
        }

        println!("{ans}");
    }

    Ok(())
}
