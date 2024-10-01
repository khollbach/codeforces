use std::{error::Error, io};

type Result<T> = std::result::Result<T, Box<dyn Error>>;

fn main() -> Result<()> {
    let mut lines = io::stdin().lines();
    let t: usize = lines.next().ok_or("t")??.parse()?;

    for _ in 1..=t {
        let l = lines.next().ok_or("l")??;
        let nsm: Vec<u32> = l
            .split_whitespace()
            .map(str::parse)
            .collect::<std::result::Result<_, _>>()?;
        let &[n, s, m] = nsm.as_slice() else {
            return Err(format!("nsm: {nsm:?}").into());
        };
        let n = n as usize;

        let tasks: Vec<(u32, u32)> = lines
            .by_ref()
            .take(n)
            .map(|l| parse_pair(&l?))
            .collect::<std::result::Result<_, _>>()?;
        if tasks.len() != n {
            return Err(format!("tasks.len(): {}", tasks.len()).into());
        }

        if can_shower(&tasks, s, m) {
            println!("YES");
        } else {
            println!("NO");
        }
    }

    if lines.next().is_some() {
        return Err("expected EOF".into());
    }

    Ok(())
}

fn parse_pair(l: &str) -> Result<(u32, u32)> {
    let nums: Vec<_> = l
        .split_whitespace()
        .map(str::parse)
        .collect::<std::result::Result<_, _>>()?;
    let &[x, y] = nums.as_slice() else {
        return Err(format!("nums.len(): {}", nums.len()).into());
    };
    Ok((x, y))
}

fn can_shower(tasks: &[(u32, u32)], shower_length: u32, day_length: u32) -> bool {
    assert!(tasks.iter().all(|t| t.0 < t.1));

    // `tasks` must be disjoint and sorted increasing.
    assert!(tasks.windows(2).all(|pair| pair[0].1 < pair[1].0));

    if let Some(last) = tasks.last() {
        assert!(last.1 <= day_length);
    }

    // Hack around edge-cases at the start and end of the day.
    let mut tasks = tasks.to_vec();
    tasks.insert(0, (0, 0));
    tasks.push((day_length, day_length));

    tasks.windows(2).any(|pair| {
        let &[t1, t2] = pair else { unreachable!() };
        let gap = t2.0 - t1.1;
        shower_length <= gap
    })
}
