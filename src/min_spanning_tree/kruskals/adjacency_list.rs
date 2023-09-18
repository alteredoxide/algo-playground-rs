//! Playing around with writing Kruskal's algorithm for building a minimum
//! spanning tree.
//! This implementation is using an adjacency list with vertex ids.
//! This avoids the complexities that are inherent in rust with a graph of
//! linked nodes, such as `Rc<RefCell<Vertex>>`.

type Edge = (usize, usize, f32);


pub struct Graph {
    pub edges: Vec<Edge>,
    pub vertices: Vec<usize>
}


fn find_set_short(vertex: usize, parents: &mut Vec<usize>) -> usize {
    if vertex != parents[vertex] {
        parents[vertex] = find_set(parents[vertex], parents);
    }
    return parents[vertex]
}


fn find_set(vertex: usize, parents: &mut Vec<usize>) -> usize {
    if vertex != parents[vertex] {
        parents[vertex] = find_set(parents[vertex], parents);
        return parents[vertex]
    }
    return vertex
}


fn union(
    mut u: usize,
    mut v: usize,
    parents: &mut Vec<usize>,
    ranks: &mut Vec<usize>
) {
    let root1 = find_set(u, parents);
    let root2 = find_set(v, parents);
    if root1 == root2 {
        return
    }
    if ranks[root1] < ranks[root2] {
        std::mem::swap(&mut u, &mut v);
    }
    parents[root2] = root1;
    if ranks[root1] == ranks[root2] {
        ranks[root1] += 1;
    }
}


pub fn build_mst(graph: Graph) -> Vec<(usize, usize)> {
    let mut forest = vec![];
    let mut parents: Vec<usize> = graph.vertices.clone();
    let mut ranks: Vec<usize> = vec![0; graph.vertices.len()];
    let mut edges_sorted: Vec<&Edge> = graph.edges
        .iter()
        .collect();
    edges_sorted.sort_unstable_by(|a, b| a.2.partial_cmp(&b.2).unwrap());
    for edge in edges_sorted {
        let root1 = find_set(edge.0, &mut parents);
        let root2 = find_set(edge.1, &mut parents);
        if root1 != root2 {
            forest.push((edge.0, edge.1));
            union(root1, root2, &mut parents, &mut ranks);
        }
    }
    forest
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
        let mut mst = super::build_mst(graph);
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
}
