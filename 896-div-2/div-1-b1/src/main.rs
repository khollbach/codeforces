use std::{error::Error, io, result::Result as StdResult};
use std::collections::HashMap;

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
        assert_eq!(nums.len(), n);

        if soln(&nums) {
            println!("YES");
        } else {
            println!("NO");
        }
    }

    Ok(())
}

fn soln(a: &[i32]) -> bool {
    let n = a.len() as u64;
    assert!(n >= 2);

    let sum: u64 = a.iter().map(|&x| x as u64).sum();
    if sum % n != 0 {
        return false; // avg is fractional
    }
    let avg = (sum / n) as i32;

    let mut flows_in = HashMap::<_, usize>::new();
    let mut flows_out = HashMap::<_, usize>::new();
    for x in a {
        let Some(Flow { in_, out }) = flow(x - avg) else {
            return false;
        };
        *flows_in.entry(in_).or_default() += 1;
        *flows_out.entry(out).or_default() += 1;
    }

    flows_in == flows_out
}

struct Flow {
    in_: usize,
    out: usize,
}

fn flow(diff: i32) -> Option<Flow> {
    if diff < 0 {
        let f = pos_flow(diff.abs() as u32)?;
        Some(Flow {
            in_: f.out,
            out: f.in_,
        })
    } else {
        pos_flow(diff as u32)
    }
}

fn pos_flow(diff: u32) -> Option<Flow> {
    // Edge-case; use a hack to just ignore these nodes.
    if diff == 0 {
        return Some(Flow { in_: 0, out: 0 });
    }

    let i = first_one(diff)?;
    let j = first_zero(diff >> i)? + i;
    if diff >> j != 0 {
        return None;
    }

    Some(Flow { 
        in_: i,
        out: j,
    })
}

fn first_one(mut n: u32) -> Option<usize> {
    for i in 0..32 {
        if n & 1 != 0 {
            return Some(i);
        }
        n >>= 1;
    }
    None
}

fn first_zero(mut n: u32) -> Option<usize> {
    for i in 0..32 {
        if n & 1 == 0 {
            return Some(i);
        }
        n >>= 1;
    }
    None
}
