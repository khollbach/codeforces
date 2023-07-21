use std::{error::Error, io, result::Result as StdResult};

type Result<T> = StdResult<T, Box<dyn Error>>;

fn main() -> Result<()> {
    let mut lines = io::stdin().lines();
    let _num_tests = lines.next().ok_or("no input")??;

    while let Some(line) = lines.next() {
        let nq: Vec<_> = line?
            .split_whitespace()
            .map(str::parse)
            .collect::<StdResult<_, _>>()?;
        let &[n, q] = nq.as_slice() else {
            Err("expected two nums: n and q")?
        };

        let nums: Vec<_> = lines
            .next()
            .ok_or("expected line of nums")??
            .split_whitespace()
            .map(str::parse)
            .collect::<StdResult<_, _>>()?;
        debug_assert_eq!(nums.len(), n);

        let array = Array::new(&nums);

        for _ in 0..q {
            let lrk: Vec<_> = lines
                .next()
                .ok_or("expected query line")??
                .split_whitespace()
                .map(str::parse)
                .collect::<StdResult<_, _>>()?;
            let &[l, r, k] = lrk.as_slice() else {
                Err("expected three nums: l, r, k")?
            };

            let start = l - 1; // 0-based
            let end = r - 1 + 1; // 0-based, exclusive
            let ans = array.parity_after_block_write(start as usize, end as usize, k);
            let ans = if ans { "YES" } else { "NO" };
            println!("{ans}");
        }
    }

    Ok(())
}

struct Array {
    /// nums.len()
    n: usize,

    /// pp[i] := parity(nums[..i])
    prefix_parities: Vec<bool>,
}

impl Array {
    fn new(nums: &[u32]) -> Self {
        let n = nums.len();
        let mut prefix_parities = Vec::with_capacity(n + 1);

        let mut acc = false;
        prefix_parities.push(acc);

        for x in nums {
            let is_odd = x % 2 == 1;
            acc ^= is_odd;

            prefix_parities.push(acc);
        }

        Self { n, prefix_parities }
    }

    fn parity(&self) -> bool {
        self.parity_of_range(0, self.n)
    }

    fn parity_of_range(&self, start: usize, end: usize) -> bool {
        self.prefix_parities[end] ^ self.prefix_parities[start]
    }

    fn parity_after_block_write(&self, start: usize, end: usize, value: u32) -> bool {
        let odd_value = value % 2 == 1;
        let odd_len = (end - start) % 2 == 1;
        let parity_of_block = odd_value && odd_len;

        self.parity() ^ self.parity_of_range(start, end) ^ parity_of_block
    }
}
