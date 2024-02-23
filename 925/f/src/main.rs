use std::{collections::HashSet, io};

fn main() {
    let mut lines = io::stdin().lines().skip(1).map(Result::unwrap);
    while let Some(nk) = lines.next() {
        let nk: Vec<usize> = nk.split_whitespace().map(|s| s.parse().unwrap()).collect();
        let &[n, k] = &nk[..] else { panic!() };

        let rows: Vec<Vec<usize>> = lines
            .by_ref()
            .take(k)
            .map(|l| l.split_whitespace().map(|s| s.parse().unwrap()).collect())
            .collect();
        let ans = soln(n, &rows);
        println!("{}", display(ans));
    }
}

fn display(ans: bool) -> &'static str {
    if ans {
        "YES"
    } else {
        "NO"
    }
}

/*
idea:
- we have n nodes, initially in an empty di-graph
- for each row, chop off the first elem
- then take the remaining elems as a linked list of edges; insert them.
- top sort the digraph
- did it work? true/false
*/

/// Di-graph on n nodes.
#[derive(Debug)]
struct Graph {
    nodes: Vec<Node>,
}

#[derive(Debug, Default, Clone)]
struct Node {
    edges: HashSet<usize>, // indices into Graph.nodes
}

fn soln(n: usize, rows: &Vec<Vec<usize>>) -> bool {
    let mut g = Graph::empty(n);
    for row in rows {
        for i in 1..row.len() - 1 {
            // skip the first number; consider adj pairs
            g.insert_edge(row[i] - 1, row[i + 1] - 1); // 0-based indexing, please.
        }
    }
    g.top_sort()
}

/*
top-sort:
- populate a collection (say stack, doesn't matter) of "exposed nodes"
    (in-degree zero)
- while pop from exposed-list:
    - add to output
    - decr all neighbors indegrees
        - update exposed list accordingly
- return true iff output is length n
*/

impl Graph {
    fn top_sort(&self) -> bool {
        let n = self.nodes.len();

        let mut indegree = vec![0; n];
        for x in 0..n {
            for &y in &self.nodes[x].edges {
                indegree[y] += 1;
            }
        }

        let mut exposed = vec![];
        for i in 0..n {
            if indegree[i] == 0 {
                exposed.push(i);
            }
        }

        while let Some(x) = exposed.pop() {
            for &y in &self.nodes[x].edges {
                indegree[y] -= 1;
                if indegree[y] == 0 {
                    exposed.push(y);
                }
            }
        }

        indegree.into_iter().all(|d| d == 0)
    }

    fn empty(n: usize) -> Self {
        Self {
            nodes: vec![Node::default(); n],
        }
    }

    /// Insert the directed edge (x, y).
    fn insert_edge(&mut self, x: usize, y: usize) {
        self.nodes[x].edges.insert(y);
    }
}
