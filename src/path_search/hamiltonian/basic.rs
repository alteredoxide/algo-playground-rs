//! A first and basic implementation of finding Hamiltonian paths in a graph.
//! NOTE: This implementation only works on an _undirected_, _connected_ graph.
//! It does _not_ need to be complete.
use std::collections::HashMap;


fn backtrack(
    graph: &HashMap<usize, Vec<usize>>,
    path: &mut Vec<usize>,
    paths: &mut Vec<Vec<usize>>
) {
    if path.len() == graph.len() {
        paths.push(path.clone());
        return
    }
    let current = path.last().expect("a path should never be empty here");
    for v in graph.get(current).expect("vertex should be in graph") {
        if !path.contains(v) {
            path.push(*v);
            backtrack(graph, path, paths);
            path.pop();
        }
    }
}


fn find_hamiltonians(graph: &HashMap<usize, Vec<usize>>) -> Vec<Vec<usize>> {
    let mut paths = vec![];
    for start in graph.keys() {
        let mut path = vec![*start];
        backtrack(graph, &mut path, &mut paths);
    }
    paths
}


#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    fn build_graph(edges: &[(usize, usize)], directed: bool) -> HashMap<usize, Vec<usize>> {
        let graph: HashMap<usize, Vec<usize>> = edges
            .into_iter()
            .fold(HashMap::new(), |mut acc, (u, v)| {
                acc.entry(*u)
                    .and_modify(|adj| adj.push(*v))
                    .or_insert(vec![*v]);
                if !directed {
                    // also get the reverse direction (we're undirected)
                    acc.entry(*v)
                        .and_modify(|adj| adj.push(*u))
                        .or_insert(vec![*u]);
                }
                acc
            });
        graph
    }

    #[test]
    fn find_paths_sm() {
        // this graph is connected but not complete
        let edges = vec![
            (1, 2),
            (2, 3),
            (2, 4),
            (3, 4),
        ];
        let graph = build_graph(&edges, false);
        let mut output = super::find_hamiltonians(&graph);
        output.sort_unstable_by(|a, b| a.cmp(&b));
        let expected = [
            [ 1, 2, 3, 4 ],
            [ 1, 2, 4, 3 ],
            [ 3, 4, 2, 1 ],
            [ 4, 3, 2, 1 ],
        ];
        assert_eq!(output, expected);
    }

    #[test]
    fn find_paths() {
        // this graph is complete
        let edges = vec![
            (1, 2),
            (1, 3),
            (1, 4),
            (2, 3),
            (2, 4),
            (3, 4),
        ];
        let graph = build_graph(&edges, false);
        let mut output = super::find_hamiltonians(&graph);
        output.sort_unstable_by(|a, b| a.cmp(&b));
        let expected = [
            [ 1, 2, 3, 4 ],
            [ 1, 2, 4, 3 ],
            [ 1, 3, 2, 4 ],
            [ 1, 3, 4, 2 ],
            [ 1, 4, 2, 3 ],
            [ 1, 4, 3, 2 ],
            [ 2, 1, 3, 4 ],
            [ 2, 1, 4, 3 ],
            [ 2, 3, 1, 4 ],
            [ 2, 3, 4, 1 ],
            [ 2, 4, 1, 3 ],
            [ 2, 4, 3, 1 ],
            [ 3, 1, 2, 4 ],
            [ 3, 1, 4, 2 ],
            [ 3, 2, 1, 4 ],
            [ 3, 2, 4, 1 ],
            [ 3, 4, 1, 2 ],
            [ 3, 4, 2, 1 ],
            [ 4, 1, 2, 3 ],
            [ 4, 1, 3, 2 ],
            [ 4, 2, 1, 3 ],
            [ 4, 2, 3, 1 ],
            [ 4, 3, 1, 2 ],
            [ 4, 3, 2, 1 ],
        ];
        assert_eq!(output, expected);
    }
}
