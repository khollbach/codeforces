use std::{cmp::Ordering, error::Error, io, ops::Not, result::Result as StdResult};

type Result<T> = StdResult<T, Box<dyn Error>>;

fn main() -> Result<()> {
    let mut lines = io::stdin().lines();
    let _num_tests: usize = lines.next().ok_or("no input")??.parse()?;

    while let Some(line) = lines.next() {
        let abc: Vec<_> = line?
            .split_whitespace()
            .map(str::parse)
            .collect::<StdResult<_, _>>()?;
        let &[a, b, c] = abc.as_slice() else {
            Err("expected 3 nums: a b c")?
        };

        let ans = match winner(a, b, c) {
            Player::A => "First",
            Player::B => "Second",
        };
        println!("{ans}");
    }

    Ok(())
}

enum Player {
    A,
    B,
}

impl Not for Player {
    type Output = Self;

    fn not(self) -> Self::Output {
        match self {
            Self::A => Self::B,
            Self::B => Self::A,
        }
    }
}

fn winner(a: u32, b: u32, c: u32) -> Player {
    let starting_player = if c % 2 == 0 { Player::A } else { Player::B };

    match a.cmp(&b) {
        Ordering::Greater => Player::A,
        Ordering::Less => Player::B,
        Ordering::Equal => !starting_player,
    }
}
