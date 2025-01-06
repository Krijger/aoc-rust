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

    pub fn minimum_distance<S, E, W>(&self, is_start: S, is_end: E, weight: W) -> Option<usize> where
        S: Fn(&T) -> bool,
        E: Fn(&T) -> bool,
        W: Fn(&T, &T) -> Option<usize>, // W gives weight of connection between two nodes, or None if not connected
    {
        self.distances_private::<S, E, W>(is_start, weight, None, None).iter()
        .find(|(node, dist)| is_end(node) && *dist < usize::MAX)
        .map(|(_, dist)| *dist)
    }

    /// Dijkstra's algorithm to calculate minumum distance between `start` and `end`.
    /// Uses https://doc.rust-lang.org/std/ptr/fn.eq.html internally to compare &Ts, which is why the start and end
    /// are supplied as functions (because raw pointers should point to graph.nodes entries, which could easily be
    /// mistaken) - in case is_start / is_end does not return true for any graph node, this function will panic
    pub fn minimum_distance_bounded<S, E, W>(&self, is_start: S, is_end: E, weight: W, lower_bound: usize) -> Option<usize> where
        S: Fn(&T) -> bool,
        E: Fn(&T) -> bool,
        W: Fn(&T, &T) -> Option<usize>, // W gives weight of connection between two nodes, or None if not connected
    {
        self.distances_private::<S, E, W>(is_start, weight, Some(Box::new(&is_end)), Some(lower_bound)).iter()
        .find(|(node, dist)| is_end(node) && *dist < usize::MAX)
        .map(|(_, dist)| *dist)
    }

    pub fn distances<S, W>(&self, is_start: S, weight: W) -> Vec<(&T, usize)> where
        S: Fn(&T) -> bool,
        W: Fn(&T, &T) -> Option<usize>,
    {
        self.distances_private::<S, Box<dyn Fn(&T) -> bool>, W>(is_start, weight, None, None)
    }
        
    fn distances_private<S, E, W>(&self, is_start: S, weight: W, end: Option<Box<&E>>, lower_bound: Option<usize>) -> Vec<(&T, usize)> where
        S: Fn(&T) -> bool,
        E: Fn(&T) -> bool,
        W: Fn(&T, &T) -> Option<usize>,
    {
        // `table` contains references to all nodes (table.0), and as prescribed by the Dijkstra's algorithm:
        // a distance (table.1), and the visited status (table.2)
        let mut table: Vec<(&T, usize, bool)> = self.nodes.iter()
            .map(|n| (n, usize::MAX, false))
            .collect();

        let start_index = table.iter().position(|(n, _, _)| is_start(n)).unwrap();
        let end_index: Option<usize> = end.map(|is_end| {
            table.iter().position(|(n, _, _)| is_end(n)).unwrap()
        });
        
        table[start_index].1 = 0;
        
        'algo: while table.iter().any(|(_, _, visited)| !visited) {
            if let Some(current_node) = table.iter()
                .filter(|(_, _, visited)| !visited )
                .fold((None, usize::MAX),
                    |(min_dist_node, min_dist), (n, d, _)| {
                        if *d < min_dist { (Some(*n), *d) } 
                        else { (min_dist_node, min_dist) }
                })
                .0 {

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

                            if end_index.is_some_and(|i| i == to_node_index) 
                            && lower_bound.is_some_and(|x| x >= to_distance_via_curr_node) {
                                // Stopping distance calculations, since end node closer than lower bound was found
                                break 'algo;
                            }
                        }
                    }
                }
            } else { // no current node means the graph is not connected and we processed all that is connected to start
                break;
            }
        }

        table.into_iter().map(|(node, dist, _)| (node, dist)).collect()
    }
}
