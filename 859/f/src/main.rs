use std::{cmp::min, error::Error, io, result::Result as StdResult};

type Result<T> = StdResult<T, Box<dyn Error>>;

fn main() -> Result<()> {
    let mut lines = io::stdin().lines();
    let _num_tests: usize = lines.next().ok_or("no input")??.parse()?;

    while let Some(line) = lines.next() {
        let (dims, start, end, dir) = read_line(&line?)?;
        let ans = num_bounces(dims, start, end, dir).unwrap_or(-1);
        println!("{ans}");
    }

    Ok(())
}

fn read_line(line: &str) -> Result<(Point, Point, Point, Dir)> {
    let mut words = line.split_whitespace();
    let (down, right) = match words.next_back().ok_or("blank line")? {
        "DR" => (true, true),
        "DL" => (true, false),
        "UR" => (false, true),
        "UL" => (false, false),
        d @ _ => Err(format!("not a dir: {d:?}"))?,
    };
    let dir = Dir { down, right };

    let nums: Vec<_> = words.map(str::parse).collect::<StdResult<_, _>>()?;
    let &[dim_i, dim_j, start_i, start_j, end_i, end_j] = nums.as_slice() else {
        Err("expected 6 nums")?
    };

    let dim = (dim_i, dim_j).into();
    let start = (start_i, start_j).into();
    let end = (end_i, end_j).into();

    Ok((dim, start, end, dir))
}

// 1-based, inclusive indexing
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Point {
    i: isize,
    j: isize,
}

impl From<(isize, isize)> for Point {
    fn from((i, j): (isize, isize)) -> Self {
        Self { i, j }
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Dir {
    down: bool,
    right: bool,
}

fn contains(dims: Point, pos: Point) -> bool {
    let i = 1 <= pos.i && pos.i <= dims.i;
    let j = 1 <= pos.j && pos.j <= dims.j;
    i && j
}

fn num_bounces(dims: Point, mut pos: Point, target: Point, mut dir: Dir) -> Option<isize> {
    // this is an upper bound on how many bounces before we get stuck in a loop
    // (two for each border tile, since we can hit it from two different angles)
    let max_bounces = 2 * (dims.i * 2 + dims.j * 2);

    for num_bounces in 0..=max_bounces {
        debug_assert!(contains(dims, pos), "{num_bounces}: {dims:?} {pos:?}");

        // are we on a collision course?
        let di = target.i - pos.i;
        let dj = target.j - pos.j;
        let target_dir = {
            let down = di >= 0;
            let right = dj >= 0;
            Dir { down, right }
        };
        let is_diagonal = di.abs() == dj.abs();
        if pos == target || dir == target_dir && is_diagonal {
            return Some(num_bounces);
        }

        // move the ball
        let vert = if dir.down { dims.i - pos.i } else { pos.i - 1 };
        let horiz = if dir.right { dims.j - pos.j } else { pos.j - 1 };
        let dist = min(vert, horiz);
        debug_assert!(
            dist >= 0,
            "{num_bounces}: {dims:?} {pos:?} {dir:?} {vert} {horiz}"
        );
        pos.i += if dir.down { dist } else { -dist };
        pos.j += if dir.right { dist } else { -dist };

        // bounce off the wall
        // (if we hit a corner, flip along both axes)
        if dist == vert {
            dir.down ^= true;
        }
        if dist == horiz {
            dir.right ^= true;
        }
    }

    None
}
