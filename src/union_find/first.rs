//! An implementation of the union-find algorithm, contained within a struct.

use std::{collections::HashMap, hash::Hash};


#[derive(Debug)]
pub struct DisjointSet<T: Clone + Eq + Hash> {
    parents: Vec<usize>,
    ranks: Vec<usize>,
    map: HashMap<T, usize>
}


impl<T: Clone + Eq + Hash> DisjointSet<T> {
    pub fn new() -> Self {
        let parents = vec![];
        let ranks = vec![];
        let map = HashMap::new();
        Self { parents, ranks, map }
    }

    pub fn get_idx(&self, value: &T) -> Option<usize> {
        if let Some(idx) = self.map.get(value) {
            return Some(*idx)
        }
        None
    }

    pub fn find(&mut self, idx: usize) -> Option<usize> {
        //if idx >= self.parents.len() { return None }
        if self.parents[idx] != idx {
            self.parents[idx] = self.find(self.parents[idx])?;
        }
        Some(self.parents[idx])
    }

    pub fn make_set(&mut self, value: &T) {
        if !self.map.contains_key(value) {
            let idx = self.parents.len();
            self.parents.push(idx);
            self.ranks.push(0);
            self.map.insert(value.clone(), idx);
        }
    }
    
    pub fn union(&mut self, a: &T, b: &T) -> Result<(), String> {
        let idx_a = match self.get_idx(a) {
            Some(i) => i,
            None => return Err(String::from("`a` not found"))
        };
        let idx_b = match self.get_idx(b) {
            Some(i) => i,
            None => return Err(String::from("`b` not found"))
        };
        // these should always be found if keys were in map
        let mut root_a = self.find(idx_a).expect("found `a` in map but not parents");
        let mut root_b = self.find(idx_b).expect("found `b` in map but not parents");
        if root_a == root_b { return Ok(()) }
        if self.ranks[root_a] < self.ranks[root_b] {
            std::mem::swap(&mut root_a, &mut root_b)
        }
        self.parents[root_b] = root_a;
        if self.ranks[root_a] == self.ranks[root_b] {
            self.ranks[root_a] += 1;
        }
        Ok(())
    }
}


pub struct Graph {
    pub edges: Vec<(usize, usize, f32)>,
    pub vertices: Vec<usize>,
}


pub fn build_mst(graph: Graph) -> Vec<(usize, usize)> {
    let mut mst = vec![];
    let mut ds = DisjointSet::<usize>::new();
    for v in graph.vertices {
        ds.make_set(&v);
    }
    let mut sorted_edges: Vec<&(usize, usize, f32)> = graph.edges.iter().collect();
    sorted_edges.sort_unstable_by(|a, b| a.2.partial_cmp(&b.2).unwrap());
    for edge in sorted_edges {
        let root_a = ds.find(edge.0).unwrap();
        let root_b = ds.find(edge.1).unwrap();
        if root_a != root_b {
            mst.push((edge.0, edge.1));
            ds.union(&root_a, &root_b).unwrap();
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
