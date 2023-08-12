use std::{cmp::min, error::Error, io, result::Result as StdResult};

type Result<T> = StdResult<T, Box<dyn Error>>;

fn main() -> Result<()> {
    let mut lines = io::stdin().lines();
    let _num_tests: usize = lines.next().ok_or("no input")??.parse()?;

    while let Some(line) = lines.next() {
        let n = line?.parse()?;

        let mut lists = Vec::with_capacity(n);
        for _ in 0..n {
            let m = lines.next().ok_or("expected m")??.parse()?;

            let list: Vec<u32> = lines
                .next()
                .ok_or("expected nums")??
                .split_whitespace()
                .map(str::parse)
                .collect::<StdResult<_, _>>()?;
            assert_eq!(list.len(), m);

            lists.push(list);
        }
        assert_eq!(lists.len(), n);

        let ans = soln(lists);
        println!("{ans}");
    }

    Ok(())
}

fn soln(mut lists: Vec<Vec<u32>>) -> u64 {
    for list in &mut lists {
        list.sort_unstable();
    }

    // min
    let mut forced = u32::MAX;
    for list in &lists {
        forced = min(forced, list[0]);
    }

    // sum
    let mut happy_sum = 0u64;
    for list in &lists {
        happy_sum += list[1] as u64;
    }

    // min
    let mut except = u32::MAX;
    for list in &lists {
        except = min(except, list[1]);
    }

    forced as u64 + happy_sum - except as u64
}
