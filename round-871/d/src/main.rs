use std::{error::Error, io, result::Result as StdResult};

type Result<T> = StdResult<T, Box<dyn Error>>;

fn main() -> Result<()> {
    let mut lines = io::stdin().lines();
    let _num_tests = lines.next().ok_or("no input")??;

    for line in lines {
        let ans = todo!();
        println!("{ans}");
    }

    Ok(())
}
