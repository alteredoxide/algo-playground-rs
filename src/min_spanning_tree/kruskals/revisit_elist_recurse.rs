//! Revisiting Kruskals algorithm for learning/memory reinforcement. This one uses the
//! recursive version of `find()` in the union-find algorithm.


struct Graph {
    edges: Vec<(usize, usize, f32)>,
    vertices: Vec<usize>,
}


fn find(idx: usize, parents: &mut [usize], ranks: &mut [usize]) -> Option<usize> {
    if idx != parents[idx] {
        parents[idx] = find(parents[idx], parents, ranks)?;
    }
    Some(parents[idx])
}


fn union(a: usize, b: usize, parents: &mut [usize], ranks: &mut [usize]) -> Option<()> {
    let mut root1 = find(a, parents, ranks)?;
    let mut root2 = find(b, parents, ranks)?;
    if root1 == root2 {
        return Some(())
    }
    if ranks[root1] < ranks[root2] {
        std::mem::swap(&mut root1, &mut root2);
    }
    if ranks[root1] == ranks[root2] {
        ranks[root1] += 1;
    }
    parents[root2] = root1;
    Some(())
}

/// The approach to Kruskal's is to utilize a disjoint set with the union-find algorithm
/// (with path compression) to build a min spanning tree from the bottom up, merging
/// subtrees at their roots, until ending up with a complete MST. But this algorithm does
/// require edges to be sorted, ascending, by weight.
/// More specifically:
/// 1. sort the edges by weight, ascending
/// 2. iterate over edges in the graph
/// 3. for each edge, get their each of their roots
/// 4. if they have different roots, merge the roots and add the edge to the MST
fn build_mst(graph: &Graph) -> Vec<(usize, usize)> {
    let mut mst = vec![];
    let mut parents = graph.vertices.clone();
    let mut ranks = vec![0_usize; parents.len()];
    let mut sorted_edges = graph.edges.clone();
    sorted_edges.sort_unstable_by(|a, b| a.2.partial_cmp(&b.2).unwrap());
    for edge in &sorted_edges {
        let root1 = find(edge.0, &mut parents, &mut ranks).unwrap();
        let root2 = find(edge.1, &mut parents, &mut ranks).unwrap();
        if root1 == root2 {
            continue
        }
        union(root1, root2, &mut parents, &mut ranks);
        mst.push((edge.0, edge.1));
    }
    mst
}


#[cfg(test)]
mod tests {

    #[test]
    fn kruskals() {
        let vertices: Vec<usize> = (0..7).collect();
        let edges = vec![
            (0, 1, 7.),
            (0, 3, 5.),
            (1, 2, 8.),
            (1, 3, 9.),
            (1, 4, 7.),
            (2, 4, 5.),
            (3, 4, 15.),
            (3, 5, 6.),
            (4, 5, 8.),
            (4, 6, 9.),
            (5, 6, 11.),
        ];
        let graph = super::Graph {
            edges,
            vertices
        };
        let mut mst = super::build_mst(&graph);
        mst.sort_by_key(|e| (e.0, e.1));
        let expected_mst = [
            (0, 1),
            (0, 3),
            (1, 4),
            (2, 4),
            (3, 5),
            (4, 6),
        ];
        assert_eq!(mst, expected_mst);
    }

    #[test]
    fn kruskals_disconnected() {
        let vertices: Vec<usize> = (0..14).collect();
        let edges = vec![
            // component 1
            (0, 1, 7.),
            (0, 3, 5.),
            (1, 2, 8.),
            (1, 3, 9.),
            (1, 4, 7.),
            (2, 4, 5.),
            (3, 4, 15.),
            (3, 5, 6.),
            (4, 5, 8.),
            (4, 6, 9.),
            (5, 6, 11.),
            // component 2
            (7, 8, 7.),
            (7, 10, 5.),
            (8, 9, 8.),
            (8, 10, 9.),
            (8, 11, 7.),
            (9, 11, 5.),
            (10, 11, 15.),
            (10, 12, 6.),
            (11, 12, 8.),
            (11, 13, 9.),
            (12, 13, 11.),
        ];
        let graph = super::Graph {
            edges,
            vertices
        };
        let mut mst = super::build_mst(&graph);
        mst.sort_by_key(|e| (e.0, e.1));
        let expected_mst = [
            // mst 1
            (0, 1),
            (0, 3),
            (1, 4),
            (2, 4),
            (3, 5),
            (4, 6),
            // mst 2
            (7, 8),
            (7, 10),
            (8, 11),
            (9, 11),
            (10, 12),
            (11, 13),
        ];
        assert_eq!(mst, expected_mst);
    }
}
