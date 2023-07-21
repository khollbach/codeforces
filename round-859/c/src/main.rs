use std::{error::Error, io, iter::zip, result::Result as StdResult};

type Result<T> = StdResult<T, Box<dyn Error>>;

fn main() -> Result<()> {
    let mut lines = io::stdin().lines();
    let _num_tests = lines.next().ok_or("no input")??;

    while let Some(line) = lines.next() {
        let _num_letters: usize = line?.parse()?;

        let s = lines.next().ok_or("expected line of letters")??;

        let ans = if can_be_2_colored(&s) { "YES" } else { "NO" };
        println!("{ans}");
    }

    Ok(())
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Color {
    Red,
    Blue,
}

fn can_be_2_colored(s: &str) -> bool {
    // mapping from letter to color.
    // initially, all letters are unassigned.
    let mut colors = [None; 26];

    // wlog, the first letter is colored red
    let alternating = [Color::Red, Color::Blue].into_iter().cycle();

    // do the greedy thing
    for (color, letter) in zip(alternating, s.chars()) {
        assert!(letter.is_ascii_lowercase());

        let offset = (letter as u8 - b'a') as usize;
        if colors[offset].is_some() && colors[offset] != Some(color) {
            // conflict! this letter already has the wrong color
            return false;
        }

        colors[offset] = Some(color);
    }

    true
}
