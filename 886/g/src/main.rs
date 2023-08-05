use std::{collections::HashMap, error::Error, io, result::Result as StdResult};

type Result<T> = StdResult<T, Box<dyn Error>>;

fn main() -> Result<()> {
    let mut lines = io::stdin().lines();
    let _num_tests: usize = lines.next().ok_or("no input")??.parse()?;

    while let Some(line) = lines.next() {
        let n = line?.parse()?;
        let points: Vec<_> = lines
            .by_ref()
            .take(n)
            .map(parse_line)
            .collect::<Result<_>>()?;
        debug_assert_eq!(points.len(), n);

        let ans = soln(&points);
        println!("{ans}");
    }

    Ok(())
}

fn parse_line(line: io::Result<String>) -> Result<Point> {
    let xy: Vec<_> = line?
        .split_whitespace()
        .map(str::parse)
        .collect::<StdResult<_, _>>()?;
    let &[x, y] = xy.as_slice() else {
        Err("expected 2 nums: x y")?
    };
    Ok((x, y))
}

type Point = (i32, i32);
type Buckets = HashMap<i32, Vec<Point>>;

fn soln(points: &[Point]) -> usize {
    let mut xs = Buckets::new();
    let mut ys = Buckets::new();
    let mut pos_slope_y_intercepts = Buckets::new();
    let mut neg_slope_y_intercepts = Buckets::new();

    for &p @ (x, y) in points {
        xs.entry(x).or_default().push(p);
        ys.entry(y).or_default().push(p);

        let y_int = y - x;
        pos_slope_y_intercepts.entry(y_int).or_default().push(p);

        let y_int = y + x;
        neg_slope_y_intercepts.entry(y_int).or_default().push(p);
    }

    let ways_to_pick_2 = |n: usize| n * n.saturating_sub(1);
    let count_buckets = |map: Buckets| map.values().map(|v| ways_to_pick_2(v.len())).sum::<usize>();

    count_buckets(xs)
        + count_buckets(ys)
        + count_buckets(pos_slope_y_intercepts)
        + count_buckets(neg_slope_y_intercepts)
}
