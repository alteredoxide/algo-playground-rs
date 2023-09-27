//! An implementation of Boruvka's algorithm that takes an edge list as input.

pub struct Graph {
    edges: Vec<(usize, usize, f32)>,
    vertices: Vec<usize>,
}


fn find_set_recursive(vertex: usize, components: &mut Vec<usize>) -> usize {
    if components[vertex] != vertex {
        components[vertex] = find_set_recursive(components[vertex], components);
    }
    components[vertex]
}


fn find_set_iterative(mut vertex: usize, components: &mut Vec<usize>) -> usize {
    let mut root = vertex;
    while components[root] != root {
        root = components[root];
    }
    while components[vertex] != root {
        let next = components[vertex];
        components[vertex] = root;
        vertex = next;
    }
    root
}


fn union_components(
    mut comp_a: usize,
    mut comp_b: usize,
    components: &mut Vec<usize>,
    ranks: &mut Vec<usize>
) {
    if ranks[comp_a] < ranks[comp_b] {
        std::mem::swap(&mut comp_a, &mut comp_b);
    }
    if ranks[comp_a] == ranks[comp_b] {
        ranks[comp_a] += 1;
    }
    components[comp_b] = comp_a;
}


pub fn build_mst(graph: Graph) -> Vec<(usize, usize)> {
    let mut mst = vec![];
    let mut components = graph.vertices.clone();
    let mut ranks: Vec<usize> = vec![1; components.len()];
    loop {
        // Initialize the cheapest edge for each component to "None"
        let mut min_edges: Vec<Option<usize>> = vec![None; components.len()];
        for (i, (u, v, weight)) in graph.edges.iter().enumerate() {
            if components[*u] == components[*v] {
                continue
            }
            match min_edges[*u] {
                None => min_edges[*u] = Some(i),
                Some(j) => {
                    let min_u = &graph.edges[j].2;
                    if weight < min_u {
                        min_edges[*u] = Some(i);
                    }
                }
            }
            match min_edges[*v] {
                None => min_edges[*v] = Some(i),
                Some(j) => {
                    let min_v = &graph.edges[j].2;
                    if weight < min_v {
                        min_edges[*v] = Some(i);
                    }
                }
            }
        }
        if min_edges.iter().all(|e| e.is_none()) {
            break
        }
        for edge_id in min_edges {
            match edge_id {
                None => continue,
                Some(i) => {
                    let e = &graph.edges[i];
                    let comp_a = find_set_iterative(e.0, &mut components);
                    let comp_b = find_set_iterative(e.1, &mut components);
                    if comp_a == comp_b {
                        continue
                    }
                    mst.push((e.0, e.1));
                    // union the two components
                    union_components(comp_a, comp_b, &mut components, &mut ranks);
                }
            }
        }
    }
    mst
}


#[cfg(test)]
mod tests {

    #[test]
    fn boruvkas() {
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

    #[test]
    fn boruvkas_disconnected() {
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
        let mut mst = super::build_mst(graph);
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
