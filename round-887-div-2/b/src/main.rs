use std::{error::Error, io, result::Result as StdResult};

type Result<T> = StdResult<T, Box<dyn Error>>;

fn main() -> Result<()> {
    let mut lines = io::stdin().lines();
    let _num_tests: usize = lines.next().ok_or("no input")??.parse()?;

    while let Some(line) = lines.next() {
        let line = line?;
        let nk: Vec<_> = line.split_whitespace().collect();
        let &[n, k] = nk.as_slice() else {
            Err("expected two words: n and k")?
        };
        let n = n.parse()?;
        let k: usize = k.parse()?;
        assert!(k >= 2);

        let k_zero_based = k - 1;
        let ans = num_fiblike_seqs(n, k_zero_based).expect("index must be non-zero");
        println!("{ans}");
    }

    Ok(())
}

fn num_fiblike_seqs(target_number: u32, target_index: usize) -> Option<usize> {
    if target_index == 0 {
        // the answer would be infinite
        return None;
    }

    // How many sequences arrive at this state?
    let num_ways = (0..=target_number)
        .filter(|&prev_number| is_possible(prev_number, target_number, target_index))
        .count();
    Some(num_ways)
}

/// Is it possible to get these two numbers an a fib-like seq, at this index?
fn is_possible(prev_number: u32, target_number: u32, target_index: usize) -> bool {
    assert_ne!(target_index, 0);

    // bad case: we can't rewind the sequence any further,
    // since prev_number is not a valid predecessor to target_number
    if prev_number > target_number {
        return false;
    }

    // base case: (prev, target) are the initial numbers
    // i.e. we reached the beginning of the sequence, and all is well.
    if target_index == 1 {
        return true;
    }

    // recursive case: deterministically rewind the sequence,
    // until we reach the beginning, or something goes wrong
    let diff = target_number - prev_number;
    is_possible(diff, prev_number, target_index - 1)
}
