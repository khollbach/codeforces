use std::{error::Error, io};

type Result<T> = std::result::Result<T, Box<dyn Error>>;

fn main() -> Result<()> {
    let mut lines = io::stdin().lines();
    let t: usize = lines.next().ok_or("t")??.parse()?;

    for _ in 1..=t {
        let l = lines.next().ok_or("l")??;
        let nums: Vec<u32> = l
            .split_whitespace()
            .map(str::parse)
            .collect::<std::result::Result<_, _>>()?;

        let &[a1, a2, b1, b2] = nums.as_slice() else {
            return Err(format!("nums.len(): {}", nums.len()).into());
        };

        let ans = a_wins(a1, a2, b1, b2);
        println!("{ans}");
    }

    if lines.next().is_some() {
        return Err("expected EOF".into());
    }

    Ok(())
}

/// How many ways can player A win?
fn a_wins(a1: u32, a2: u32, b1: u32, b2: u32) -> usize {
    let possible_games = [
        (a1, a2, b1, b2),
        (a1, a2, b2, b1),
        (a2, a1, b1, b2),
        (a2, a1, b2, b1),
    ];
    possible_games
        .into_iter()
        .filter(|&(a1, a2, b1, b2)| a_win(a1, a2, b1, b2))
        .count()
}

/// Does player A win if the cards are played in this order?
fn a_win(a1: u32, a2: u32, b1: u32, b2: u32) -> bool {
    let a_points = (a1 > b1) as usize + (a2 > b2) as usize;
    let b_points = (b1 > a1) as usize + (b2 > a2) as usize;
    a_points > b_points
}
