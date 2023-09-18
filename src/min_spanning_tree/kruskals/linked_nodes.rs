//! Playing around with writing Kruskal's algorithm for building a minimum
//! spanning tree.
//! This submodule is using linked nodes, or a linked graph, much like a linked
//! list.

use std::{rc::Rc, cell::RefCell};

type VertexCell = Rc<RefCell<Vertex>>;


pub struct Graph {
    pub edges: Vec<Edge>,
    pub vertices: Vec<VertexCell>
}


pub struct Edge {
    pub from: VertexCell,
    pub to: VertexCell,
    pub weight: f32
}


impl From<((usize, usize, f32), &Vec<VertexCell>)> for Edge {
    fn from(item: ((usize, usize, f32), &Vec<VertexCell>)) -> Self {
        let e = item.0;
        let vertices = item.1;
        Self {
            from: vertices[e.0].clone(),
            to: vertices[e.1].clone(),
            weight: e.2,
        }
    }
}


pub struct Vertex {
    pub id: usize,
    pub parent: Option<VertexCell>,
    pub rank: usize
}


impl Vertex {
    pub fn new(id: usize) -> Self {
        Self {
            id,
            parent: None,
            rank: 0,
        }
    }
}


fn make_set(vertex: VertexCell) {
    let mut vertex_mut = vertex.borrow_mut();
    if vertex_mut.parent.is_none() {
        vertex_mut.parent = Some(vertex.clone());
    }
}


/// The goal of find_set is to find and return the root of the connected
/// component to which the given vertex belongs. It accomplishes this by
/// recursively moving up the tree until it finds the vertex that is it's own
/// parent. It also updates the parents of each vertex along the way.
fn find_set(vertex: VertexCell) -> VertexCell {
    let parent = {
        vertex.borrow()
            .parent
            .as_ref()
            .expect("Every vertex should have a parent")
            .clone()
    };
    let mut vertex_mut = vertex.borrow_mut();
    if !Rc::ptr_eq(&vertex, &parent) {
        let parent = find_set(parent.clone());
        vertex_mut.parent = Some(parent.clone());
        return parent
    }
    drop(vertex_mut);
    vertex
}


fn union(u: VertexCell, v: VertexCell) {
    let root1 = find_set(u);
    let root2 = find_set(v);
    if Rc::ptr_eq(&root1, &root2) {
        return
    }
    let mut root1_mut = root1.borrow_mut();
    let mut root2_mut = root2.borrow_mut();
    if root1_mut.rank < root2_mut.rank {
        std::mem::swap(&mut root1_mut, &mut root2_mut);
    }
    root1_mut.parent = Some(root2.clone());
    if root1_mut.rank == root2_mut.rank {
        root1_mut.rank += 1;
    }
}


pub fn build_mst(graph: &mut Graph) -> Vec<(usize, usize)> {
    let mut mst = vec![];
    for v in &graph.vertices {
        make_set(v.clone());
    }

    graph.edges.sort_by(|a, b| a.weight.partial_cmp(&b.weight).unwrap());
    for e in &mut graph.edges {
        let root1 = find_set(e.from.clone());
        let root2 = find_set(e.to.clone());
        if !Rc::ptr_eq(&root1, &root2) {
            mst.push((e.from.borrow().id, e.to.borrow().id));
            union(root1, root2);
        }
    }
    mst
}


#[cfg(test)]
mod tests {
    use std::rc::Rc;
    use std::cell::RefCell;
    use super::{Graph, Edge, Vertex, VertexCell};

    #[test]
    fn kruskals() {
        let vertices: Vec<VertexCell> = (0..7).map(|i| {
            let v = Vertex::new(i);
            Rc::new(RefCell::new(v))
        }).collect();
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
        let edges = edges
            .into_iter()
            .map(|e| Edge::from((e, &vertices)))
            .collect();
        let mut graph = Graph {
            edges,
            vertices
        };
        let mut mst = super::build_mst(&mut graph);
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
