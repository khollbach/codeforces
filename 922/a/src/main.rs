use std::{error::Error, io};

fn main() -> Result<(), Box<dyn Error>> {
    for l in io::stdin().lines().skip(1) {
        let hw: Vec<u32> = l?
            .split_whitespace()
            .map(|s| s.parse().expect("parse"))
            .collect();
        let &[h, w] = hw.as_slice() else {
            panic!("length");
        };
        let ans = soln(h, w);
        println!("{ans}");
    }
    Ok(())
}

fn soln(height: u32, width: u32) -> u32 {
    width / 2 * height
}
