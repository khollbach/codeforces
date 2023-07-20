use std::{error::Error, io};

fn main() -> Result<(), Box<dyn Error>> {
    let mut lines = io::stdin().lines();
    let _num_tests = lines.next().ok_or("no input")??;

    for line in lines {
        let ans = todo!();
        println!("{ans}");
    }

    Ok(())
}
