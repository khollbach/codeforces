use std::{error::Error, io, result::Result as StdResult, cmp::min};

type Result<T> = StdResult<T, Box<dyn Error>>;

fn main() -> Result<()> {
    let mut lines = io::stdin().lines();
    let _num_tests: usize = lines.next().ok_or("no input")??.parse()?;

    while let Some(line) = lines.next() {
        let nkab: Vec<_> = line?
            .split_whitespace()
            .map(str::parse)
            .collect::<StdResult<_, _>>()?;
        let &[n, k, a, b] = nkab.as_slice() else {
            Err("expected 4 nums")?
        };
        assert!(k <= n);
        let points: Vec<_> = lines
            .by_ref()
            .take(n)
            .map(parse_point)
            .collect::<Result<_>>()?;
        assert_eq!(points.len(), n);

        // 0-indexed, please
        let ans = soln(&points, k, a - 1, b - 1);

        println!("{ans}");
    }

    Ok(())
}

fn parse_point(line: io::Result<String>) -> Result<(i32, i32)> {
    let xy: Vec<_> = line?
        .split_whitespace()
        .map(str::parse)
        .collect::<StdResult<_, _>>()?;
    let &[x, y] = xy.as_slice() else {
        Err("expected 2 coords")?
    };
    Ok((x, y))
}

fn dist((x1, y1): (i32, i32), (x2, y2): (i32, i32)) -> u64 {
    x1.abs_diff(x2) as u64 + y1.abs_diff(y2) as u64
}

fn soln(points: &[(i32, i32)], k: usize, a: usize, b: usize) -> u64 {
    let direct = dist(points[a], points[b]);
    if k == 0 {
        return direct;
    }

    let mut city_near_a = (i32::MIN, i32::MIN);
    let mut min_dist = u64::MAX;
    for i in 0..k {
        let d = dist(points[i], points[a]);
        if d < min_dist {
            min_dist = d;
            city_near_a = points[i];
        }
    }

    let mut city_near_b = (i32::MIN, i32::MIN);
    let mut min_dist = u64::MAX;
    for i in 0..k {
        let d = dist(points[i], points[b]);
        if d < min_dist {
            min_dist = d;
            city_near_b = points[i];
        }
    }

    let via_cities = dist(points[a], city_near_a) + dist(city_near_b, points[b]);

    min(via_cities, direct)
}
