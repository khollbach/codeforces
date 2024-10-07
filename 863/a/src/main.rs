use std::{error::Error, io};

type Result<T> = std::result::Result<T, Box<dyn Error>>;

fn main() -> Result<()> {
    let mut lines = io::stdin().lines();
    let t: usize = lines.next().ok_or("t")??.parse()?;

    for _ in 0..t {
        let l = lines.next().ok_or("nd")??;
        let mut words = l.split_whitespace();
        let n: usize = words.next().ok_or("n")?.parse()?;
        let d: &str = words.next().ok_or("d")?;
        assert_eq!(d.len(), 1);
        let d = d.chars().next().unwrap();
        assert!(words.next().is_none());

        let num = lines.next().ok_or("num")??;
        assert_eq!(num.len(), n);

        let ans = largest_number(&num, d);
        println!("{ans}");
    }

    assert!(lines.next().is_none());

    Ok(())
}

fn largest_number(num: &str, bonus_digit: char) -> String {
    let mut bonus_digit = Some(bonus_digit);
    let mut out = String::with_capacity(num.len() + 1);

    // Find the first digit less than bonus_digit, and insert
    // it before that point.
    for d in num.chars() {
        if bonus_digit.is_some() && d < bonus_digit.unwrap() {
            out.push(bonus_digit.unwrap());
            bonus_digit = None;
        }
        out.push(d);
    }

    if let Some(d) = bonus_digit {
        out.push(d);
    }

    out
}
