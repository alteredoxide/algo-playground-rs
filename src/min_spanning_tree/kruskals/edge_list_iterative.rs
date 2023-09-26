//! A more memory-efficient implementation of Kruskal's algoritm compared to the one
//! found in [super::edge_list_recursive]: this one requires constant memory.

pub struct Graph {
    pub edges: Vec<(usize, usize, f32)>,
    pub vertices: Vec<usize>,
}


fn find_set(mut vertex: usize, parents: &mut Vec<usize>) -> usize {
    let mut root = vertex;
    while parents[root] != root {
        root = parents[root];
    }
    while parents[vertex] != root {
        let parent = parents[vertex];
        parents[vertex] = root;
        vertex = parent;
    }
    parents[root]
}


fn union(a: usize, b: usize, parents: &mut Vec<usize>, ranks: &mut Vec<usize>) {
    let mut root1 = find_set(a, parents);
    let mut root2 = find_set(b, parents);
    if root1 == root2 { return }
    if ranks[root1] < ranks[root2] {
        std::mem::swap(&mut root1, &mut root2);
    }
    parents[root2] = root1;
    if ranks[root1] == ranks[root2] {
        ranks[root1] += 1;
    }
}


pub fn build_mst(graph: Graph) -> Vec<(usize, usize)> {
    let mut mst = vec![];
    let size = graph.vertices.len();
    // init each vertex as it's own parent
    let mut parents: Vec<usize> = (0..size).collect();
    let mut ranks: Vec<usize> = vec![1; size];
    // 1. sort edges
    let mut sorted_edges: Vec<&(usize, usize, f32)> = graph.edges.iter().collect();
    sorted_edges.sort_unstable_by(|a, b| a.2.partial_cmp(&b.2).unwrap());
    for edge in sorted_edges {
        let root1 = find_set(edge.0, &mut parents);
        let root2 = find_set(edge.1, &mut parents);
        if root1 != root2 {
            mst.push((edge.0, edge.1));
            union(root1, root2, &mut parents, &mut ranks);
        }
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
