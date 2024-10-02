use std::{error::Error, io};

type Result<T> = std::result::Result<T, Box<dyn Error>>;

fn main() -> Result<()> {
    let mut lines = io::stdin().lines();
    let t: usize = lines.next().ok_or("t")??.parse()?;

    for _ in 0..t {
        let n: usize = lines.next().ok_or("n")??.parse()?;

        let points: Vec<(u32, u32)> = lines
            .by_ref()
            .take(n)
            .map(|l| parse_point(&l?))
            .collect::<std::result::Result<_, _>>()?;
        let cols = points_to_columns(&points);

        let ans = count_right_triangles(&cols);
        println!("{ans}");
    }

    if lines.next().is_some() {
        return Err("expected EOF".into());
    }

    Ok(())
}

fn parse_point(l: &str) -> Result<(u32, u32)> {
    let nums: Vec<u32> = l
        .split_whitespace()
        .map(str::parse)
        .collect::<std::result::Result<_, _>>()?;
    let &[x, y] = nums.as_slice() else {
        return Err(format!("expected 2 nums, got: {}", nums.len()).into());
    };
    Ok((x, y))
}

fn points_to_columns(points: &[(u32, u32)]) -> Vec<Column> {
    // let n = 2 * 10_usize.pow(5);
    let n = points.len();
    let mut columns = vec![Column::default(); n + 1];

    for &(x, y) in points {
        let c = &mut columns[x as usize];
        match y {
            0 => c.bot = true,
            1 => c.top = true,
            _ => panic!("invalid y: {y}"),
        }
    }

    columns
}

#[derive(Debug, Clone, Default)]
struct Column {
    top: bool,
    bot: bool,
}

fn count_right_triangles(cols: &[Column]) -> u64 {
    count_axis_aligned(cols) + count_45_degree(cols)
}

fn count_axis_aligned(cols: &[Column]) -> u64 {
    let n = cols.len();
    if n == 0 {
        return 0;
    }

    let mut num_points = 0;
    for c in cols {
        if c.top {
            num_points += 1;
        }
        if c.bot {
            num_points += 1;
        }
    }

    let mut count = 0;
    for i in 0..n { // BUG: had bounds wrong
        if cols[i].top && cols[i].bot {
            count += num_points - 2;
        }
    }
    count
}

fn count_45_degree(cols: &[Column]) -> u64 {
    let n = cols.len();
    if n == 0 {
        return 0;
    }

    let mut count = 0;
    for i in 1..n - 1 {
        if cols[i].top && cols[i - 1].bot && cols[i + 1].bot {
            count += 1;
        }
        if cols[i].bot && cols[i - 1].top && cols[i + 1].top {
            count += 1;
        }
    }
    count
}
