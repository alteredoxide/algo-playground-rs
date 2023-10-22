//! A revisit to Prim's algorithm. This is not meant to be an improvement; it's only meant
//! to be a re-implementation for memory reinforcement.

/// The approach taken by Prim's algorithm is a greedy one:
/// 1. Start with a randomly selected starting node.
/// 2. Look at all edges to vertices that have not yet been visited, and select the one
///    with the smallest weight.
/// 3. Add the edge to mst.
/// 4. Once all vertices have been visited (included in the mst), return mst.
fn build_mst(input: &[Vec<(usize, f32)>]) -> Vec<(usize, usize)> {
    let size: usize = input.len();
    let mut mst = vec![];
    // NOTE: I would use a HashSet instead of a Vec for visited, if I thought there would
    // be a sufficiently large number (on the order of 100 or more) of vertices in the
    // graph.
    // "randomly" select starting node as the first
    let mut visited = vec![false; size];
    let mut n_visited: usize = 1;
    visited[0] = true;
    while n_visited < size {
        for u in 0..size {
            // We want edge (left, right) to be (visited, not visited)
            // This ensures an efficient means of revisiting the left vertices, in order
            // to explore paths not yet traversed.
            if !visited[u] { continue }
            let mut min_v: Option<usize> = None;
            let mut min_w = f32::INFINITY;
            for e in &input[u] {
                if visited[e.0] { continue }
                if e.1 < min_w {
                    min_v = Some(e.0);
                    min_w = e.1;
                }
            }
            if let Some(min_v) = min_v {
                mst.push((u, min_v));
                visited[min_v] = true;
                n_visited += 1;
            } 
        }
    }
    mst
}


#[cfg(test)]
mod tests {

    #[test]
    fn prims() {
        let adj_list = vec![
            vec![(1, 7.), (3, 5.)],
            vec![(2, 8.), (3, 9.), (4, 7.)],
            vec![(4, 5.)],
            vec![(4, 15.), (5, 6.)],
            vec![(5, 8.), (6, 9.)],
            vec![(6, 11.)],
            vec![(5, 11.)]
        ];
        let mut mst = super::build_mst(&adj_list);
        mst.sort_by_key(|e| (e.0, e.1));
        // NOTE: this solution looks different from the one in Kruskals because the
        // algorithms are different, and Prim's selection of an arbitrary starting node
        // will affect the solution.
        let expected_mst = [
            (0, 1),
            (0, 3),
            (1, 2),
            (1, 4),
            (3, 5),
            (5, 6),
        ];
        assert_eq!(mst, expected_mst);
    }
}
