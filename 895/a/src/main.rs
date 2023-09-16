use std::{error::Error, io, result::Result as StdResult};

type Result<T> = StdResult<T, Box<dyn Error>>;

fn main() -> Result<()> {
    let mut lines = io::stdin().lines();
    let _num_tests: usize = lines.next().ok_or("no input")??.parse()?;

    while let Some(line) = lines.next() {
        let abc: Vec<_> = line?.split_whitespace().map(str::parse).collect::<StdResult<_, _>>()?;
        let &[a, b, c] = abc.as_slice() else {
            Err("expected 3 nums")?
        };

        let ans = soln(a, b, c);
        println!("{ans}");
    }

    Ok(())
}

fn soln(a: f32, b: f32, c: f32) -> usize {
    let mut num_moves = 0;

    let delta = (a - b).abs();
    let mut amount_to_move = delta / 2.;

    while amount_to_move != 0. {
        let amt = f32::min(amount_to_move, c);
        amount_to_move -= amt;
        num_moves += 1;
    }

    num_moves
}
