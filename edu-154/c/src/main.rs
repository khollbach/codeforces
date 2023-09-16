use std::{error::Error, io, result::Result as StdResult};

type Result<T> = StdResult<T, Box<dyn Error>>;

fn main() -> Result<()> {
    let mut lines = io::stdin().lines();
    let _num_tests: usize = lines.next().ok_or("no input")??.parse()?;

    while let Some(line) = lines.next() {
        let ans = if soln(&line?) { "YES" } else { "NO" };
        println!("{ans}");
    }

    Ok(())
}

fn soln(s: &str) -> bool {
    let mut stack = vec![];

    for c in s.chars() {
        match c {
            '+' => stack.push(0), // 0 represents "?"
            '-' => {
                stack.pop().expect("pop empty stack");
            }
            '1' => {
                // Mark the "?"s on the top of the stack as "sorted".
                for i in (0..stack.len()).rev() {
                    match stack[i] {
                        -1 => return false,
                        0 => stack[i] = 1,
                        1 => break,
                        _ => unreachable!(),
                    }
                }
            }
            '0' => {
                if stack.len() < 2 {
                    return false;
                }
                let top = stack.last_mut().unwrap();
                if *top == 1 {
                    return false;
                }
                *top = -1;
            }
            _ => panic!("invalid op: {c:?}"),
        }
    }

    true
}
