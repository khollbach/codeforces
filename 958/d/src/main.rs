fn main() {
    println!("Hello, world!");
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct NodeId(u32);

#[derive(Debug)]
struct Forest {
    nodes: HashMap<NodeId, Node>,
}

#[derive(Debug)]
struct Node {
    value: u64,
    neighbors: Vec<NodeId>,
}

impl Forest {
    fn ans(mut self) -> u32;

    fn value_sum(&self) -> u32;

    fn attack_forest(&mut self) {
        let mut visited = HashSet::new();
        let mut to_kill = Vec::new();

        for id in self.nodes.keys() {
            if !visited.contains(id) {
                self.attack_tree(id, true, &mut visited, &mut to_kill);
            }
        }

        for id in to_kill {
            self.kill(id);
        }
    }

    fn attack_tree(
        &self,
        root: NodeId,
        allowed_to_kill_root: bool,
        visited: &mut HashSet<NodeId>,
        to_kill: &mut Vec<NodeId>,
    ) {
        visited.insert(root);

        let mut child_sum = 0;
        for nbr in self.nodes[root].neighbors {
            if !visited.contains(nbr) {
                child_sum += self.nodes[nbr].value;
            }
        }

        // We decide whether to kill the root based on a greedy heuristic.
        // We don't prove it here, but we claim that the resulting strategy
        // is optimal. The off-by-ones matter: you must break ties by killing
        // the root.

        // JUST KIDDING. It's not optimal:
        //
        //    4
        //   / \
        //  2   3
        //      |
        //      4
        //
        // The current heuristic would cause us to spare the root,
        // but that's not optimal. Optimal strat is to kill both 4s in the
        // first round.
        //
        // I think we want instead to do a DP thing where we compute, for
        // each subtree: the cost if we're allowed to kill the root of that
        // subtree, and the cost if we're not allowed to kill its root.
        //
        // I haven't worked out the details; it might help to pretend that
        // the game only lasts two rounds, and then generalize the approach
        // once you've figured out that. (Q: does the game *ever* go longer
        // than say 2, 3, 4 rounds?)

        let mut allowed_to_kill_children = true;
        if self.nodes[root].value >= child_sum {
            to_kill.push(root);
            allowed_to_kill_children = false;
        }

        for nbr in self.nodes[root].neighbors {
            if !visited.contains(nbr) {
                self.attack_tree(nbr, allowed_to_kill_children, visited, to_kill);
            }
        }
    }
}
