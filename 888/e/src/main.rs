use std::{
    cmp::min,
    error::Error,
    io,
    iter::{self, zip},
    result::Result as StdResult,
};

type Result<T> = StdResult<T, Box<dyn Error>>;

fn main() -> Result<()> {
    let mut lines = io::stdin().lines();
    let _num_tests: usize = lines.next().ok_or("no input")??.parse()?;

    while let Some(line) = lines.next() {
        let nk: Vec<_> = line?
            .split_whitespace()
            .map(str::parse)
            .collect::<StdResult<_, _>>()?;
        let &[n, k] = nk.as_slice() else {
            Err("expected 2 nums: n k")?
        };

        let costs: Vec<_> = lines
            .next()
            .ok_or("expected line: costs")??
            .split_whitespace()
            .map(str::parse)
            .collect::<StdResult<_, _>>()?;
        assert_eq!(costs.len(), n);

        let free: Vec<_> = lines
            .next()
            .ok_or("expected line: free")??
            .split_whitespace()
            .map(|s| {
                let i: usize = str::parse(s)?;
                Ok(i - 1)
            })
            .collect::<Result<_>>()?;
        assert_eq!(free.len(), k);

        let edge_groups: Vec<_> = lines
            .by_ref()
            .take(n)
            .map(parse_edge_group)
            .collect::<Result<_>>()?;
        assert_eq!(edge_groups.len(), n);

        let edges = zip(edge_groups, 0..).flat_map(|(group, dest)| zip(group, iter::repeat(dest)));

        let ans = soln(costs, &free, edges);
        for (i, min_cost) in ans.into_iter().enumerate() {
            if i != 0 {
                print!(" ");
            }
            print!("{min_cost}");
        }
        println!();
    }

    Ok(())
}

fn parse_edge_group(line: io::Result<String>) -> Result<Vec<usize>> {
    let line = line?;
    let mut nums = line.split_whitespace().map(str::parse);

    let m = nums.next().ok_or("empty line")??;
    let edges: Vec<_> = nums.map(|i| Ok(i? - 1)).collect::<Result<_>>()?;
    assert_eq!(edges.len(), m);

    Ok(edges)
}

fn soln(
    mut costs: Vec<u64>,
    free: &[usize],
    edges: impl Iterator<Item = (usize, usize)>,
) -> Vec<u64> {
    for &i in free {
        costs[i] = 0;
    }

    let dag = Dag::new(costs.len(), edges);
    dag.min_costs(&costs)
}

// ---

#[derive(Debug)]
struct Dag {
    nodes: Vec<Node>,
}

#[derive(Debug, Clone, Default)]
struct Node {
    out_edges: Vec<usize>,
    in_edges: Vec<usize>,
}

impl Node {
    fn new() -> Self {
        Self::default()
    }
}

impl Dag {
    fn new(n: usize, edges: impl Iterator<Item = (usize, usize)>) -> Self {
        let mut nodes = vec![Node::new(); n];

        for (i, j) in edges {
            nodes[i].out_edges.push(j);
            nodes[j].in_edges.push(i);
        }

        Self { nodes }
    }

    /// Return a permutation of [0, n) s.t. for all i < j,
    /// the node `perm[i]` cannot be reached from `perm[j]`.
    fn top_sort(&self) -> Vec<usize> {
        let n = self.nodes.len();

        let mut in_degrees = vec![0; n];
        for i in 0..n {
            in_degrees[i] = self.nodes[i].in_edges.len();
        }

        // A "source" is a node with indegree 0.
        let mut sources = Vec::with_capacity(n);
        for i in 0..n {
            if in_degrees[i] == 0 {
                sources.push(i);
            }
        }

        let mut order = Vec::with_capacity(n);
        while let Some(s) = sources.pop() {
            // "Delete" s and all its outgoing edges.
            // This might expose some new sources.
            for &i in &self.nodes[s].out_edges {
                in_degrees[i] -= 1;
                if in_degrees[i] == 0 {
                    sources.push(i);
                }
            }

            order.push(s);
        }

        assert_eq!(order.len(), n, "cycle detected");

        order
    }

    fn min_costs(&self, costs: &[u64]) -> Vec<u64> {
        let n = self.nodes.len();
        assert_eq!(n, costs.len());

        const NONE: u64 = u64::MAX;
        let mut min_costs = vec![NONE; n];

        // top_sort ensures each node's dependencies are ready in time.
        for i in self.top_sort() {
            debug_assert_eq!(min_costs[i], NONE);
            let in_edges = &self.nodes[i].in_edges;
            debug_assert!(in_edges.iter().all(|&j| min_costs[j] != NONE));

            // You can either:
            // * buy this potion from the store, or
            // * mix it from its recipe.

            let buy_cost = costs[i];

            let combine_cost = if in_edges.is_empty() {
                // edge-case: there's no recipe for this potion
                u64::MAX
            } else {
                in_edges.iter().map(|&j| min_costs[j]).sum()
            };

            min_costs[i] = min(buy_cost, combine_cost);
        }

        debug_assert!(!min_costs.contains(&NONE));
        min_costs
    }
}
