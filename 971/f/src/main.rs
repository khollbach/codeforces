use std::{error::Error, io};

type Result<T> = std::result::Result<T, Box<dyn Error>>;

fn main() -> Result<()> {
    let mut lines = io::stdin().lines();
    let t: usize = lines.next().ok_or("t")??.parse()?;

    for _ in 0..t {
        let l = lines.next().ok_or("nq")??;
        let (n, q) = parse_pair(&l)?;

        let l = lines.next().ok_or("nums")??;
        let nums = parse_nums(&l)?;
        assert_eq!(nums.len(), n);

        let queries = parse_queries(lines.by_ref().take(q))?;
        if queries.len() != q {
            return Err(format!(
                "unexpected EOF during queries. want {} got {}",
                queries.len(),
                q
            )
            .into());
        }

        let m = Matrix::new(nums);
        let answers = answer_queries(&m, &queries);
        for ans in answers {
            println!("{ans}");
        }
    }

    if lines.next().is_some() {
        return Err("expected EOF".into());
    }

    Ok(())
}

fn answer_queries(m: &Matrix, queries: &[Query]) -> Vec<u64> {
    let n = m.nums.len();
    let mut out = Vec::with_capacity(n);

    for q in queries {
        assert!(q.l <= q.r);
        let (li, lj) = m.index_to_coords(q.l);
        let (ri, rj) = m.index_to_coords(q.r);

        let ans = if li == ri {
            m.slice_sum(li, lj, rj)
        } else {
            let prefix = m.slice_sum(li, lj, n);
            let suffix = m.slice_sum(ri, 0, rj);
            let num_rows_between = (ri - li - 1) as u64;
            let full_row_sum = m.slice_sum(0, 0, n);
            prefix + suffix + num_rows_between * full_row_sum
        };

        out.push(ans);
    }

    out
}

#[derive(Debug)]
struct Matrix {
    nums: Vec<u64>,
    /// sum(nums[0..i])
    prefix_sums: Vec<u64>,
    /// sum(nums[i..n])
    suffix_sums: Vec<u64>,
}

impl Matrix {
    fn new(nums: Vec<u64>) -> Self {
        let n = nums.len();
        let mut prefix_sums = vec![0; n + 1];
        let mut suffix_sums = vec![0; n + 1];
        for i in 0..n {
            prefix_sums[i + 1] = prefix_sums[i] + nums[i];
        }
        for i in (0..n).rev() {
            suffix_sums[i] = suffix_sums[i + 1] + nums[i];
        }
        Self {
            nums,
            prefix_sums,
            suffix_sums,
        }
    }

    /// Sum a slice of row 0.
    ///
    /// Exclusive of end.
    fn simple_slice_sum(&self, start: usize, end: usize) -> u64 {
        assert!(start <= end);
        self.prefix_sums[end] - self.prefix_sums[start]
    }

    /// Exclusive of col_end.
    fn slice_sum(&self, row: usize, col_start: usize, col_end: usize) -> u64 {
        if col_start == col_end {
            return 0;
        }

        let n = self.nums.len();
        let start = (col_start + row) % n;
        let end = (col_end + row) % n;

        if start < end {
            self.simple_slice_sum(start, end)
        } else {
            self.simple_slice_sum(start, n) + self.simple_slice_sum(0, end)
        }
    }

    fn index_to_coords(&self, idx: usize) -> (usize, usize) {
        let n = self.nums.len();
        let row = idx / n;
        let col = idx % n;
        (row, col)
    }
}

/// 0-based, left-inclusive.
#[derive(Debug, Clone, Copy)]
struct Query {
    l: usize,
    r: usize,
}

fn parse_queries(lines: impl Iterator<Item = io::Result<String>>) -> Result<Vec<Query>> {
    lines.map(|l| parse_query(&l?)).collect()
}

fn parse_query(l: &str) -> Result<Query> {
    let (i, j) = parse_pair(l)?;
    let q = Query {
        l: i.checked_sub(1).ok_or("i=0")?,
        r: j, // BUG: had -1
    };
    Ok(q)
}

fn parse_pair(l: &str) -> Result<(usize, usize)> {
    let nums: Vec<_> = l
        .split_whitespace()
        .map(str::parse)
        .collect::<std::result::Result<_, _>>()?;
    let &[x, y] = nums.as_slice() else {
        return Err(format!("expected 2 nums, got: {}", nums.len()).into());
    };
    Ok((x, y))
}

fn parse_nums(l: &str) -> Result<Vec<u64>> {
    let nums: Vec<_> = l
        .split_whitespace()
        .map(str::parse)
        .collect::<std::result::Result<_, _>>()?;
    Ok(nums)
}
