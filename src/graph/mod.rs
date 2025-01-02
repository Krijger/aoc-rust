use std::ptr::eq;

/// A non-directed graph.
/// In Rust, a generic graph is impossible: for more information see
/// https://stackoverflow.com/questions/32300132/why-cant-i-store-a-value-and-a-reference-to-that-value-in-the-same-struct
/// https://stackoverflow.com/questions/28608823/how-to-model-complex-recursive-data-structures-graphs
/// The alternative is to make a struct owning the nodes only and methods to the needed graphing
/// specifically optimised for our algorithm.
pub struct Graph<T>
{
    pub nodes: Vec<T>,
}

impl <T> Graph<T>
{
    pub fn new() -> Self {
        Graph { nodes: Vec::new() }
    }
    
    fn connections<W>(&self, from: &T, weigth: &W) -> Vec<(&T, usize)> where 
        W: Fn(&T, &T) -> Option<usize>, // W gives weight of connection between two nodes, or None if not connected
    {
        self.nodes.iter()
        .filter_map(|n| weigth(from, n).map(|w| (n, w)))
        .collect()
    }

    /// Dijkstra's algorithm to calculate minumum distance between `start` and `end`.
    /// Uses https://doc.rust-lang.org/std/ptr/fn.eq.html internally to compare &Ts, which is why the start and end
    /// are supplied as functions (because raw pointers should point to graph.nodes entries, which could easily be
    /// mistaken) - in case is_start / is_end does not return true for any graph node, this function will panic
    pub fn minimum_distance<S, E, W>(&self, is_start: S, is_end: E, weight: W) -> usize where
        S: Fn(&T) -> bool,
        E: Fn(&T) -> bool,
        W: Fn(&T, &T) -> Option<usize>, // W gives weight of connection between two nodes, or None if not connected
    {
        // `table` contains references to all nodes (table.0), and as prescribed by the Dijkstra's algorithm:
        // a distance (table.1), and the visited status (table.2)
        let mut table: Vec<(&T, usize, bool)> = self.nodes.iter()
            .map(|n| (n, usize::MAX, false))
            .collect();

        let start_index = table.iter().position(|(n, _, _)| is_start(n)).unwrap();
        table[start_index].1 = 0;
        
        while table.iter().any(|(_, _, visited)| !visited) {
            let current_node = table.iter()
                .filter(|(_, _, visited)| !visited )
                .fold((None, usize::MAX),
                    |(min_dist_node, min_dist), (n, d, _)| {
                        if *d < min_dist { (Some(*n), *d) } 
                        else { (min_dist_node, min_dist) }
                })
                .0.unwrap();

            let current_node_index = table.iter().position(|(n, _, _)| eq(*n, current_node)).unwrap();
            table[current_node_index].2 = true;

            // for all connections to current node, check if the route via the current node is the first or
            // in case a route already exists, whether the current route is shorter. In both cases, update
            // the distance of the connected node in our table
            let curr_dist = table[current_node_index].1;
            for (to, w) in self.connections(current_node, &weight) {
                let to_node_index = table.iter().position(|(n, _, _)| eq(*n, to)).unwrap();
                if !table[to_node_index].2 {
                    let to_distance_via_curr_node = curr_dist + w;
                    if to_distance_via_curr_node < table[to_node_index].1 {
                        table[to_node_index].1 = to_distance_via_curr_node;
                    }
                }
            }
        }

        let end_index = table.iter().position(|(n, _, _)| is_end(n)).unwrap();
        table[end_index].1
    }
}
