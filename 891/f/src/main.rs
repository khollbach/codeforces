use std::{collections::HashMap, error::Error, io, result::Result as StdResult};

type Result<T> = StdResult<T, Box<dyn Error>>;

fn main() -> Result<()> {
    let mut lines = io::stdin().lines();
    let _num_tests: usize = lines.next().ok_or("no input")??.parse()?;

    while let Some(line) = lines.next() {
        let n = line?.parse()?;

        let nums: Vec<_> = lines
            .next()
            .ok_or("expected nums")??
            .split_whitespace()
            .map(str::parse)
            .collect::<StdResult<_, _>>()?;
        assert_eq!(nums.len(), n);

        let q = lines.next().ok_or("expected num_queries")??.parse()?;
        let queries: Vec<_> = lines
            .by_ref()
            .take(q)
            .map(parse_query)
            .collect::<Result<_>>()?;
        assert_eq!(queries.len(), q);

        let all_answers = soln(&nums, &queries);
        for (i, query_answer) in all_answers.into_iter().enumerate() {
            if i != 0 {
                print!(" ");
            }
            print!("{query_answer}");
        }
        println!();
    }

    Ok(())
}

fn parse_query(line: io::Result<String>) -> Result<Query> {
    let sp: Vec<_> = line?
        .split_whitespace()
        .map(str::parse)
        .collect::<StdResult<_, _>>()?;
    let &[sum, product] = sp.as_slice() else {
        Err("expected 2 nums: sum, product")?
    };
    Ok(Query { sum, product })
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Query {
    sum: i64,
    product: i64,
}

// TODO: this doesn't work and I don't know why...

fn soln(nums: &[i64], queries: &[Query]) -> Vec<usize> {
    let mut freqs = HashMap::<_, usize>::with_capacity(nums.len());
    for &x in nums {
        *freqs.entry(x).or_default() += 1;
    }

    let mut answers = Vec::with_capacity(queries.len());
    for &q in queries {
        let ans = match solve_quadratic_eqn(1, -q.sum, q.product) {
            Ok((x, y)) if x == y => n_choose_2(*freqs.get(&x).unwrap_or(&0)),
            Ok((x, y)) => freqs.get(&x).unwrap_or(&0) * freqs.get(&y).unwrap_or(&0),
            Err(_) => 0,
        };
        answers.push(ans);
    }
    answers
}

/// Output integer solutions, if they exist.
fn solve_quadratic_eqn(a: i64, b: i64, c: i64) -> Result<(i64, i64)> {
    let a = a as i128;
    let b = b as i128;
    let c = c as i128;

    // Solve for `a * x^2 + b * x + c == 0`, using the formula:
    // (-b + sqrt(b^2 - 4ac)) / 2a

    let sqrt_d = {
        let d = b.pow(2) - 4 * a * c;
        if d < 0 {
            Err(format!("d negative: {d}"))?
        }
        let sqrt_d = (d as f64).sqrt();
        if sqrt_d.fract() != 0. {
            Err(format!("sqrt(d) fractional: {sqrt_d}"))?
        }
        sqrt_d.trunc() as i128
    };

    let x1 = {
        let top = -b - sqrt_d;
        let bot = 2 * a;
        if top % bot != 0 {
            Err(format!("-b-sqrt(d) not divisible by 2a: {top} / {bot}"))?
        }
        top / bot
    };

    let x2 = {
        let top = -b + sqrt_d;
        let bot = 2 * a;
        if top % bot != 0 {
            Err(format!("-b+sqrt(d) not divisible by 2a: {top} / {bot}"))?
        }
        top / bot
    };

    Ok((x1.try_into()?, x2.try_into()?))
}

fn n_choose_2(n: usize) -> usize {
    if n == 0 {
        return 0;
    }

    n * (n - 1) / 2
}
