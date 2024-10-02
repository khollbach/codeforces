use std::{
    cmp::{max, min, Reverse},
    collections::{BinaryHeap, HashMap, HashSet},
    error::Error,
    io,
};

type Result<T> = std::result::Result<T, Box<dyn Error>>;

fn main() -> Result<()> {
    let mut lines = io::stdin().lines();
    let t: usize = lines.next().ok_or("t")??.parse()?;

    for _ in 0..t {
        let nmh: Vec<usize> = lines
            .next()
            .ok_or("nmh")??
            .split_whitespace()
            .map(str::parse)
            .collect::<std::result::Result<_, _>>()?;
        let &[n, m, h] = nmh.as_slice() else {
            panic!("expected 3 nums nmh, got {}", nmh.len());
        };

        let horses: Vec<usize> = lines
            .next()
            .ok_or("a_1, ..., a_h")??
            .split_whitespace()
            .map(str::parse)
            .collect::<std::result::Result<_, _>>()?;
        assert_eq!(horses.len(), h);

        let edges: Vec<(usize, usize, u64)> = lines
            .by_ref()
            .take(m)
            .map(|l| parse_edge(&l?))
            .collect::<std::result::Result<_, _>>()?;
        assert_eq!(edges.len(), m);

        let g = Graph::new(n, &horses, &edges);
        if let Some(ans) = g.fastest_meeting(0, n - 1) {
            println!("{ans}");
        } else {
            println!("-1");
        }
    }

    assert!(lines.next().is_none());
    Ok(())
}

fn parse_edge(l: &str) -> Result<(usize, usize, u64)> {
    let words: Vec<_> = l.split_whitespace().collect();
    let &[u, v, w] = words.as_slice() else {
        return Err(format!("expected 3 words, got {}", words.len()).into());
    };
    let u = u.parse()?;
    let v = v.parse()?;
    let w = w.parse()?;
    Ok((u, v, w))
}

impl Graph {
    /// Warning: inputs (horses and edges) use 1-based indexing.
    fn new(n: usize, horses: &[usize], edges: &[(usize, usize, u64)]) -> Self {
        let mut nodes = vec![Node::default(); n];

        for h in horses {
            nodes[h - 1].has_horse = true;
        }

        for &(u, v, w) in edges {
            nodes[u - 1].edges.push((v - 1, w));
            nodes[v - 1].edges.push((u - 1, w));
        }

        Self { nodes }
    }
}

#[derive(Debug)]
struct Graph {
    nodes: Vec<Node>,
}

#[derive(Debug, Default, Clone)]
struct Node {
    has_horse: bool,
    /// Indices into Graph.nodes, and weights.
    edges: Vec<(usize, u64)>,
}

impl Graph {
    fn fastest_meeting(&self, start_1: usize, start_2: usize) -> Option<u64> {
        // dbg!(self, start_1, start_2); // TODO

        let paths_1 = self.shortest_paths(start_1);
        // dbg!(&paths_1); // TODO
        let paths_2 = self.shortest_paths(start_2);
        // dbg!(&paths_2); // TODO

        let mut min = None;

        for i in 0..self.nodes.len() {
            if let (Some(d1), Some(d2)) = (paths_1[i], paths_2[i]) {
                let time_to_meet = max(d1, d2);

                if min.is_none() || time_to_meet < min.unwrap() {
                    min = Some(time_to_meet);
                }
            }
        }

        min
    }

    fn shortest_paths(&self, start: usize) -> Vec<Option<u64>> {
        let h = self.shortest_horse_paths(start);
        // dbg!(&h); // TODO
        let p = self.shortest_paths_no_horses(start);
        // dbg!(&p); // TODO

        let n = self.nodes.len();
        let mut out = Vec::with_capacity(n);
        for i in 0..n {
            let min = match (h[i], p[i]) {
                (Some(x), Some(y)) => Some(min(x, y)),
                (Some(x), None) => Some(x),
                (None, Some(y)) => Some(y),
                (None, None) => None,
            };
            out.push(min);
        }
        out
    }

    fn shortest_horse_paths(&self, start: usize) -> Vec<Option<u64>> {
        let n = self.nodes.len();
        let mut out = vec![None; n];

        let mut to_visit = BinaryHeap::new();
        to_visit.push((Reverse(0), start, self.nodes[start].has_horse));

        let mut visited = HashSet::new();

        let mut dist_estimates = HashMap::new();
        dist_estimates.insert((start, self.nodes[start].has_horse), 0);

        while let Some((Reverse(dist), curr, horse)) = to_visit.pop() {
            if visited.contains(&(curr, horse)) {
                continue;
            }
            visited.insert((curr, horse));

            if horse {
                out[curr] = Some(dist);
            }

            for &(node, weight) in &self.nodes[curr].edges {
                let w = if horse { weight / 2 } else { weight };
                let d = dist + w;
                let h = horse || self.nodes[node].has_horse;
                if !dist_estimates.contains_key(&(node, h)) || d < dist_estimates[&(node, h)] {
                    dist_estimates.insert((node, h), d);
                    to_visit.push((Reverse(d), node, h));
                }
            }
        }

        out
    }

    fn shortest_paths_no_horses(&self, start: usize) -> Vec<Option<u64>> {
        let n = self.nodes.len();
        let mut out = vec![None; n];

        let mut to_visit = BinaryHeap::new();
        to_visit.push((Reverse(0), start));

        let mut dist_estimates = HashMap::new();
        dist_estimates.insert(start, 0);

        while let Some((Reverse(dist), curr)) = to_visit.pop() {
            if out[curr].is_some() {
                // Already visited.
                continue;
            }

            out[curr] = Some(dist);

            for &(node, weight) in &self.nodes[curr].edges {
                let d = dist + weight;
                if !dist_estimates.contains_key(&node) || d < dist_estimates[&node] {
                    dist_estimates.insert(node, d);
                    to_visit.push((Reverse(d), node));
                }
            }
        }

        out
    }
}
