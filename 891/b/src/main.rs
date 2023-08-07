use std::{error::Error, io, result::Result as StdResult};

type Result<T> = StdResult<T, Box<dyn Error>>;

fn main() -> Result<()> {
    let mut lines = io::stdin().lines();
    let _num_tests: usize = lines.next().ok_or("no input")??.parse()?;

    while let Some(line) = lines.next() {
        let mut s = line?;
        soln(&mut s);
        println!("{s}");
    }

    Ok(())
}

fn soln(s: &mut String) {
    assert!(s.chars().all(|c| c.is_ascii_digit()));

    // Safety: we only write ASCII digits.
    let s = unsafe { s.as_mut_vec() };

    s.reverse();
    s.push(b'0');

    let mut truncate_to = 0; // exclusive

    for i in 0..s.len() - 1 {
        // Round up if you can.
        if s[i] >= b'5' && s[i + 1] != b'9' {
            s[i + 1] += 1;
            truncate_to = i + 1;
        }
    }

    for i in 0..truncate_to {
        s[i] = b'0';
    }

    if s.last() == Some(&b'0') {
        s.pop();
    }
    s.reverse()
}
