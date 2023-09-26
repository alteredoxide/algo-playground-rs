//! An implementation of Prim's algorithm using a weighted adjacency list
//! as inputs.


/// A more standard approach to Prim's algorithm using an adjacency list as input.
pub fn build_mst_standard(adj_list: Vec<Vec<(usize, f32)>>) -> Vec<(usize, usize)> {
    let mut mst = vec![];
    let size: usize = adj_list.len();
    let mut visited = vec![false; adj_list.len()];
    let mut visited_count: usize = 1;
    visited[0] = true;
    while visited_count < adj_list.len() {
        for left in 0..size {
            if !visited[left] { continue }
            let mut min_weight = f32::INFINITY;
            let mut right: usize = left;
            for (vertex, weight) in &adj_list[left] {
                if visited[*vertex] { continue }
                if weight < &min_weight {
                    min_weight = *weight;
                    right = *vertex;
                }
            }
            if left != right {
                visited[right] = true;
                visited_count += 1;
                mst.push((left, right));
            }
        }
    }
    mst
}


/// This one is just built in a freestyle manner: Just trying something different and,
/// in this case, getting a look at what an overly-complex implementation could look like.
pub fn build_mst_freestyle(adj_list: Vec<Vec<(usize, f32)>>) -> Vec<(usize, usize)> {
    let mut mst = vec![];
    let mut visited = vec![false; adj_list.len()];
    let mut visited_count: usize = 1;
    let mut last_visited: usize = 0;  // arbitrary choice of node
    let mut depleted: Vec<usize> = vec![];
    visited[last_visited] = true;
    while visited_count < adj_list.len() {
        if let Some(edge) = get_lowest_cost_edge(&visited, &adj_list, last_visited) {
            last_visited = edge.1;
            visited[edge.1] = true;
            visited_count += 1;
            mst.push(edge);
        } else {
            depleted.push(last_visited);
            for (i, is_visited) in visited
                .iter()
                .enumerate()
                .filter(|(i, _)| !&depleted.contains(&i))
            {
                if *is_visited && i != last_visited {
                    last_visited = i;
                    break
                }
            }
        }
    }
    mst
}


fn get_lowest_cost_edge(
    visited: &[bool],
    adj_list: &Vec<Vec<(usize, f32)>>,
    left: usize
)
    -> Option<(usize, usize)>
{
    let mut out = None;
    let mut min_weight: f32 = f32::INFINITY;
    for (right, weight) in &adj_list[left] {
        if visited[*right] {
            continue
        }
        if weight < &min_weight {
            min_weight = *weight;
            out = Some((left, *right))
        }
    }
    out
}



#[cfg(test)]
mod tests {

    #[test]
    fn prims_standard() {
        let adj_list = vec![
            vec![(1, 7.), (3, 5.)],
            vec![(2, 8.), (3, 9.), (4, 7.)],
            vec![(4, 5.)],
            vec![(4, 15.), (5, 6.)],
            vec![(5, 8.), (6, 9.)],
            vec![(6, 11.)],
            vec![(5, 11.)]
        ];
        let mut mst = super::build_mst_standard(adj_list);
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

    #[test]
    fn prims_freestyle() {
        let adj_list = vec![
            vec![(1, 7.), (3, 5.)],
            vec![(2, 8.), (3, 9.), (4, 7.)],
            vec![(4, 5.)],
            vec![(4, 15.), (5, 6.)],
            vec![(5, 8.), (6, 9.)],
            vec![(6, 11.)],
            vec![(5, 11.)]
        ];
        let mut mst = super::build_mst_freestyle(adj_list);
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
