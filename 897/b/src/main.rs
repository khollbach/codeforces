use std::{error::Error, io, result::Result as StdResult};

type Result<T> = StdResult<T, Box<dyn Error>>;

fn main() -> Result<()> {
    let mut lines = io::stdin().lines();
    let _num_tests: usize = lines.next().ok_or("no input")??.parse()?;

    while let Some(line) = lines.next() {
        let n = line?.parse()?;

        let s: String = lines.next().ok_or("expected s")??;
        let s: Vec<u8> = s.into_bytes();
        let s: Vec<bool> = s
            .into_iter()
            .map(|c| match c {
                b'0' => false,
                b'1' => true,
                _ => panic!("not an ASCII 0 or 1: {c:?}"),
            })
            .collect();
        assert_eq!(s.len(), n);

        for b in soln(&s) {
            print!("{}", b as u8);
        }
        println!();
    }

    Ok(())
}

fn soln(s: &[bool]) -> Vec<bool> {
    assert!(!s.is_empty());
    let n = s.len();

    let min = diffs(s);
    let max = n - min;

    let mut out = vec![false; n + 1];

    let mut i = min;
    while i <= max {
        out[i] = true;

        i += if n % 2 == 0 { 2 } else { 1 };
    }

    out
}

fn diffs(s: &[bool]) -> usize {
    if s.is_empty() {
        return 0;
    }
    let n = s.len();

    let mut diffs = 0;

    let mut i = 0;
    let mut j = n - 1;
    while i < j {
        if s[i] != s[j] {
            diffs += 1;
        }
        i += 1;
        j -= 1;
    }

    diffs
}
