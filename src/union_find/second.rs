//! Second time implementing union-find.


struct UnionFind {
    parents: Vec<usize>,
    ranks: Vec<usize>,
}


impl UnionFind {
    fn new(n_vertices: usize) -> Self {
        Self {
            parents: (0..n_vertices).collect(),
            ranks: vec![0; n_vertices],
        }
    }

    fn find(&mut self, idx: usize) -> Option<usize> {
        let parent_idx = self.parents.get(idx)?;
        if parent_idx != &idx {
            self.parents[idx] = self.find(self.parents[idx])?;
            return Some(self.parents[idx])
        }
        Some(*parent_idx)
    }

    fn union(&mut self, a: usize, b: usize) -> Result<(), String> {
        let mut root1 = match self.find(a) {
            None => return Err(String::from("unable to find `a`")),
            Some(root) => root,
        };
        let mut root2 = match self.find(b) {
            None => return Err(String::from("unable to find `b`")),
            Some(root) => root,
        };
        if root1 == root2 { return Ok(()) }
        match self.ranks[root1].cmp(&self.ranks[root2]) {
            std::cmp::Ordering::Less => std::mem::swap(&mut root1, &mut root2),
            std::cmp::Ordering::Equal=> self.ranks[root1] += 1,
            _ => {}
        }
        self.parents[root2] = root1;
        Ok(())
    }
}


struct Graph {
    vertices: Vec<usize>,
    edges: Vec<(usize, usize, f32)>,
}


fn build_mst(graph: Graph) -> Vec<(usize, usize)> {
    let mut mst = Vec::<(usize, usize)>::with_capacity(graph.vertices.len());
    let mut sorted_edges = graph.edges.clone();
    sorted_edges.sort_unstable_by(|a, b| a.2.partial_cmp(&b.2).unwrap());
    let mut uf = UnionFind::new(graph.vertices.len());
    for e in sorted_edges {
        let root1 = uf.find(e.0).unwrap();
        let root2 = uf.find(e.1).unwrap();
        if root1 == root2 { continue }
        uf.union(root1, root2).unwrap();
        mst.push((e.0, e.1));
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
