use std::{error::Error, io, result::Result as StdResult};

type Result<T> = StdResult<T, Box<dyn Error>>;

fn main() -> Result<()> {
    let mut lines = io::stdin().lines();
    let num_tests: usize = lines.next().ok_or("no input")??.parse()?;

    for t in 1..=num_tests {
        let line = lines.next().ok_or("line")?;
        let p = line?.parse()?;
        if let Some(ans) = soln(p) {
            print!("Case #{t}: ");
            print_soln(ans);
            println!();
        } else {
            println!("Case #{t}: -1");
        }
    }

    Ok(())
}

fn print_soln(f: [u32; 42]) {
    let mut list = vec![];
    for i in 0..42 {
        for _ in 0..f[i] {
            list.push(i);
        }
    }

    print!("{} ", list.len());
    for i in list {
        print!("{i} ");
    }
}

fn soln(mut p: u32) -> Option<[u32; 42]> {
    assert_ne!(p, 0);
    let mut factors = [0; 42];
    for i in 2..=41 {
        while p % i == 0 {
            factors[i as usize] += 1;
            p /= i;
        }
    }
    if p != 1 {
        return None; // p has factors larger than 41
    }
    // dbg!(factors);

    let mut out = vec![];
    solns(&mut factors, &mut out);
    // dbg!(&out, out.len());
    let min_count = out.iter().map(count).min()?;
    out.into_iter().find(|f| count(f) == min_count)
}

fn count(f: &[u32; 42]) -> usize {
    let mut count = 0;
    for i in 0..42 {
        count += f[i] as usize;
    }
    // dbg!(count);
    count
}

fn sum(f: &[u32; 42]) -> u32 {
    let mut sum = 0;
    for i in 0..42 {
        sum += i * f[i as usize];
    }
    sum
}

fn solns(f: &mut [u32; 42], out: &mut Vec<[u32; 42]>) {
    let sum = sum(f);
    if sum >= 42 {
        return;
    }
    if sum == 41 {
        out.push(f.clone());
        return;
    }

    let mut cheater_answer = f.clone();
    cheater_answer[1] = 41 - sum;
    out.push(cheater_answer);

    for i in 0..42 {
        if f[i] == 0 {
            continue;
        }
        f[i] -= 1;
        for j in i..42 { // OBO! whoops.
            if f[j] == 0 {
                continue;
            }
            if i * j >= 42 {
                break;
            }
            f[j] -= 1;
            f[i * j] += 1;
            solns(f, out);
            f[i * j] -= 1;
            f[j] += 1;
        }
        f[i] += 1;
    }
}
