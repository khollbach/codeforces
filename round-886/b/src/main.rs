use std::{error::Error, io, result::Result as StdResult, str::FromStr};

type Result<T> = StdResult<T, Box<dyn Error>>;

fn main() -> Result<()> {
    let mut lines = io::stdin().lines();
    let _num_tests: usize = lines.next().ok_or("no input")??.parse()?;

    while let Some(line) = lines.next() {
        let n: usize = line?.parse()?;
        if n == 0 {
            Err("n can't be 0")?
        }

        let responses: Vec<_> = lines
            .by_ref()
            .take(n)
            .map(|line| Ok(line?.parse()?))
            .collect::<Result<_>>()?;
        if responses.len() != n {
            Err("expected n responses")?
        }

        let ans = best_short_response(&responses).expect("n is nonzero");
        let one_based = ans + 1;
        println!("{one_based}");
    }

    Ok(())
}

struct Response {
    length: u8,
    quality: u8,
}

impl FromStr for Response {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self> {
        let nums: Vec<_> = s
            .split_whitespace()
            .map(str::parse)
            .collect::<StdResult<_, _>>()?;
        let &[length, quality] = nums.as_slice() else {
            Err("expected two numbers: length and quality")?
        };

        Ok(Response { length, quality })
    }
}

fn best_short_response(responses: &[Response]) -> Option<usize> {
    let short_responses = responses.iter().enumerate().filter(|(_, r)| r.length <= 10);
    let best = short_responses.max_by_key(|(_, r)| r.quality);
    best.map(|(i, _)| i)
}
