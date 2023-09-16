use std::{error::Error, io, result::Result as StdResult};

type Result<T> = StdResult<T, Box<dyn Error>>;

fn main() -> Result<()> {
    let mut lines = io::stdin().lines();
    let _num_tests: usize = lines.next().ok_or("no input")??.parse()?;

    while let Some(line) = lines.next() {
        let naq: Vec<_> = line?.split_whitespace().map(str::parse).collect::<StdResult<_, _>>()?;
        let &[n, a, q] = naq.as_slice() else {
            Err("expected 3 nums")?
        };

        let events = lines.next().ok_or("expected events")??;
        assert_eq!(events.len(), q);

        let ans = soln(n, a, &events);
        println!("{ans}");
    }

    Ok(())
}

fn soln(n: usize, mut a: usize, events: &str) -> &'static str {
    assert!(a <= n);

    let mut max_possible_uniq = a;

    const EOF: char = '$';
    for e in events.chars().chain([EOF]) {
        if a == n {
            return "YES";
        }

        match e {
            '+' => {
                a += 1;
                max_possible_uniq += 1;
            }
            '-' => {
                a -= 1;
            }
            EOF => (),
            _ => panic!("invalid event: {e:?}"),
        }
        debug_assert!(a <= n);
    }

    if max_possible_uniq >= n {
        "MAYBE"
    } else {
        "NO"
    }
}
