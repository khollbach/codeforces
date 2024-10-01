use std::{error::Error, io};

type Result<T> = std::result::Result<T, Box<dyn Error>>;

fn main() -> Result<()> {
    let mut lines = io::stdin().lines();
    let t: usize = lines.next().ok_or("t")??.parse()?;

    for _ in 1..=t {
        let s = lines.next().ok_or("s")??;
        let t = lines.next().ok_or("t")??;

        if let Some(ans) = contains_substring(&s, &t) {
            println!("YES");
            println!("{ans}");
        } else {
            println!("NO");
        }
    }

    if lines.next().is_some() {
        return Err("expected EOF".into());
    }

    Ok(())
}

/// Treat '?'s in `s` as wildcards.
fn contains_substring(s: &str, t: &str) -> Option<String> {
    assert!(s.is_ascii());
    assert!(t.is_ascii());
    let s = s.as_bytes();
    let t = t.as_bytes();
    let mut out = String::with_capacity(s.len());

    let mut i = 0;
    let mut j = 0;

    while i < s.len() && j < t.len() {
        if s[i] == b'?' || s[i] == t[j] {
            out.push(t[j] as char);
            j += 1;
        } else {
            out.push(s[i] as char);
        }
        i += 1;
    }

    // Consume the rest of s so that the output string is complete.
    while i < s.len() {
        if s[i] == b'?' {
            out.push('z');
        } else {
            out.push(s[i] as char);
        }
        i += 1;
    }

    // Did we consume all of t?
    // BUG: had i instead of j
    if j == t.len() {
        Some(out)
    } else {
        None
    }
}
