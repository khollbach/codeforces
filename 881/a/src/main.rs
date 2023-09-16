use std::{error::Error, io, result::Result as StdResult};

type Result<T> = StdResult<T, Box<dyn Error>>;

fn main() -> Result<()> {
    let mut lines = io::stdin().lines();
    let _num_tests: usize = lines.next().ok_or("no input")??.parse()?;

    while let Some(line) = lines.next() {
        // dbg!(line);

        let _n: usize = line?.parse()?;

        let next_line = lines.next().ok_or("expected nums")??;
        let nums: Vec<_> = next_line
            .split_whitespace()
            .map(str::parse)
            .collect::<StdResult<Vec<_>, _>>()?;

        let ans = soln(nums);
        println!("{ans}");
    }

    Ok(())
}

// find median element in O(n)
// "quickselect" ...

fn soln(mut nums: Vec<u32>) -> u32 {
    nums.sort();

    // 0 1 2 3   n=4  n/2=2
    //     ^

    // 0 1 2 3 4   n=5  n/2=2.5=2
    //     ^
    //     mid

    let mid = nums.len() / 2;

    let small = nums[..mid].iter().sum::<u32>();
    let large = nums[nums.len() - mid..].iter().sum::<u32>();
    large - small
}
