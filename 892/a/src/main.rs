use std::{error::Error, io, result::Result as StdResult};

type Result<T> = StdResult<T, Box<dyn Error>>;

fn main() -> Result<()> {
    let mut lines = io::stdin().lines();
    let _num_tests: usize = lines.next().ok_or("no input")??.parse()?;

    while let Some(line) = lines.next() {
        let n = line?.parse()?;

        let nums: Vec<_> = lines
            .next()
            .ok_or("expected nums")??
            .split_whitespace()
            .map(str::parse)
            .collect::<StdResult<_, _>>()?;
        debug_assert_eq!(nums.len(), n);

        match soln(&nums) {
            None => println!("-1"),
            Some((b, c)) => {
                println!("{} {}", b.len(), c.len());
                print_nums(&b);
                print_nums(&c);
            }
        }
    }

    Ok(())
}

fn print_nums(nums: &[u32]) {
    for (i, x) in nums.iter().enumerate() {
        if i != 0 {
            print!(" ");
        }
        print!("{x}");
    }
    println!();
}

fn soln(nums: &[u32]) -> Option<(Vec<u32>, Vec<u32>)> {
    let smallest = nums.iter().min().unwrap();
    if nums.iter().all(|x| x == smallest) {
        return None;
    }
    Some(nums.iter().copied().partition(|x| x == smallest))
}
