use std::{
    error::Error,
    io::{self, Write},
    result::Result as StdResult,
};

type Result<T> = StdResult<T, Box<dyn Error>>;

fn main() -> Result<()> {
    let mut lines = io::stdin().lines();
    let _num_tests: usize = lines.next().ok_or("no input")??.parse()?;

    while let Some(line) = lines.next() {
        let n: usize = line?.parse()?;

        let nums: Vec<_> = lines
            .next()
            .ok_or("expected line of nums")??
            .split_whitespace()
            .map(str::parse)
            .collect::<StdResult<_, _>>()?;
        debug_assert_eq!(nums.len(), n);

        // Unlock stdin, to allow queries to access it.
        drop(lines);
        let ans = find_heavy_pile(&nums, 0, nums.len());
        lines = io::stdin().lines();

        let one_based = ans + 1;
        println!("! {one_based}");
    }

    Ok(())
}

fn find_heavy_pile(nums: &[u32], i: usize, j: usize) -> usize {
    let len = j - i;
    if len <= 1 {
        debug_assert_eq!(len, 1);

        // This should be the heavy pile.
        let weight = query(i, j);
        let expected = nums[i];
        debug_assert_eq!(weight, expected + 1);

        return i;
    }

    let mid = i + len / 2;
    let weight = query(i, mid);
    let expected = nums[i..mid].iter().sum();

    if weight > expected {
        find_heavy_pile(nums, i, mid)
    } else {
        debug_assert!(weight <= expected);
        find_heavy_pile(nums, mid, j)
    }
}

fn query(i: usize, j: usize) -> u32 {
    let len = j - i;
    print!("? {len}");
    for idx in i..j {
        let one_based = idx + 1;
        print!(" {one_based}");
    }
    println!();
    io::stdout().flush().unwrap();

    io::stdin()
        .lines()
        .next()
        .unwrap()
        .unwrap()
        .parse()
        .unwrap()
}
