use std::{collections::HashMap, error::Error, io, result::Result as StdResult};

type Result<T> = StdResult<T, Box<dyn Error>>;

fn main() -> Result<()> {
    let mut lines = io::stdin().lines();
    let _num_tests: usize = lines.next().ok_or("no input")??.parse()?;

    while let Some(line) = lines.next() {
        let nm: Vec<_> = line?
            .split_whitespace()
            .map(str::parse)
            .collect::<StdResult<_, _>>()?;
        let &[n, m] = nm.as_slice() else {
            Err("expected two nums: n and m")?
        };
        debug_assert!(m <= n);

        let edges: Vec<_> = lines
            .by_ref()
            .take(m)
            .map(parse_line)
            .collect::<Result<_>>()?;
        debug_assert_eq!(edges.len(), m);

        let graph = Graph::new(n, edges.into_iter());
        let ans = if graph.is_possible() { "YES" } else { "NO" };
        println!("{ans}");
    }

    Ok(())
}

fn parse_line(line: io::Result<String>) -> Result<Edge> {
    let ijd: Vec<_> = line?
        .split_whitespace()
        .map(str::parse)
        .collect::<StdResult<_, _>>()?;
    let &[i, j, d] = ijd.as_slice() else {
        Err("expected 3 nums: i j d")?
    };

    // 1-indexed -> 0-indexed.
    // Also they're writing (u -> v) as `v u` for some reason.
    let from = j as usize - 1;
    let to = i as usize - 1;

    Ok(Edge { from, to, dist: d })
}

struct Edge {
    from: usize,
    to: usize,
    dist: i64,
}

struct Graph {
    nodes: Vec<Node>,
}

#[derive(Clone)]
struct Node {
    edges: Vec<(usize, i64)>,
}

impl Graph {
    fn new(num_nodes: usize, edges: impl Iterator<Item = Edge>) -> Self {
        let mut nodes = vec![Node { edges: vec![] }; num_nodes];

        for e in edges {
            nodes[e.from].edges.push((e.to, e.dist));
            nodes[e.to].edges.push((e.from, -e.dist));
        }

        Self { nodes }
    }

    fn is_possible(&self) -> bool {
        if self.nodes.is_empty() {
            return true;
        }

        let n = self.nodes.len();
        let mut positions = HashMap::with_capacity(n);
        let mut q = Vec::with_capacity(n);

        // Traverse each connected component.
        for start in 0..n {
            if positions.contains_key(&start) {
                continue;
            }

            // wlog, the first node has absolute position 0
            positions.insert(start, 0);
            q.push(start);

            while let Some(i) = q.pop() {
                for &(j, d) in &self.nodes[i].edges {
                    let pos = positions[&i] + d;
                    if let Some(&existing_pos) = positions.get(&j) {
                        if existing_pos != pos {
                            return false;
                        }
                    } else {
                        positions.insert(j, pos);
                        q.push(j);
                    }
                }
            }
        }

        true
    }
}
