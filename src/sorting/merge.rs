//! Implementations of merge sort.
//! TODO: Show time and memory complexity analyses for each implementation.

use std::fmt::Debug;


pub fn sort_mutate<T: PartialOrd + Clone + Debug>(values: &mut [T]) {
    if values.len() <= 1 {
        return
    }
    let mid = values.len() / 2;
    sort_mutate(&mut values[..mid]);
    sort_mutate(&mut values[mid..]);
    // create a copy of the original to pull values from during updates
    let v = values.to_vec();
    let (mut i, mut j, mut current) = (0_usize, mid, 0_usize);
    while i < mid && j < values.len() {
        if &v[i] <= &v[j] {
            values[current] = v[i].clone();
            i += 1;
        } else {
            values[current] = v[j].clone();
            j += 1;
        }
        current += 1;
    }
    if i < mid {
        values[current..].clone_from_slice(&v[i..mid]);
    } else if j < values.len() {
        values[current..].clone_from_slice(&v[j..]);
    }
}


/// Used by `sort_clone()`
fn merge<T>(a: Vec<T>, b: Vec<T>) -> Vec<T>
    where T: PartialEq + PartialOrd + Clone + Debug
{
    let mut out = Vec::with_capacity(a.len() + b.len());
    // first check if the the smallest value in one sequence is greater than the largest
    // value in the other sequence
    if a.last() < b.first() {
        out.extend(a);
        out.extend(b);
        return out
    } else if b.last() < a.first() {
        out.extend(b);
        out.extend(a);
        return out
    }
    let mut i: usize = 0;
    let mut j: usize = 0;
    while i < a.len() {
        let x = &a[i];
        while j < b.len() {
            let y = &b[j];
            if y < x {
                out.push(y.clone());
                j += 1;
            } else {
                break
            }
        }
        out.push(x.clone());
        i += 1;
    }
    if j < b.len() {
        out.extend_from_slice(&b[j..]);
    }
    out
}


pub fn sort_clone<T>(values: &[T]) -> Vec<T>
    where T: PartialEq + PartialOrd + Clone + Debug
{
    // base case
    if values.len() == 1 {
        return values.iter().cloned().collect()
    }
    let mid = (values.len() + 1) / 2;
    let a_: Vec<T> = values[..mid].into();
    let b_: Vec<T> = values[mid..].into();
    let a = sort_clone(&a_);
    let b = sort_clone(&b_);
    return merge(a, b)
}


#[cfg(test)]
mod tests {
    #[test]
    fn sort_mutate() {
        let mut cases: Vec<Vec<i32>> = vec![
            vec![0, 2, 1, 4, 7, 3],
            vec![2, -1, 5, 2, 9],
            vec![0, 1, 2, 4, 3, 0],
        ];
        let expected: Vec<Vec<i32>> = vec![
            vec![0, 1, 2, 3, 4, 7],
            vec![-1, 2, 2, 5, 9],
            vec![0, 0, 1, 2, 3, 4],
        ];
        for (case, expect) in cases.iter_mut().zip(&expected) {
            super::sort_mutate(case);
            assert_eq!(case, expect);
        }
    }

    #[test]
    fn sort_clone() {
        let mut cases: Vec<Vec<i32>> = vec![
            vec![0, 2, 1, 4, 7, 3],
            vec![2, -1, 5, 2, 9],
            vec![0, 1, 2, 4, 3, 0],
        ];
        let expected: Vec<Vec<i32>> = vec![
            vec![0, 1, 2, 3, 4, 7],
            vec![-1, 2, 2, 5, 9],
            vec![0, 0, 1, 2, 3, 4],
        ];
        for (case, expect) in cases.iter_mut().zip(&expected) {
            let result = super::sort_clone(case);
            assert_eq!(&result, expect);
        }
    }
}
