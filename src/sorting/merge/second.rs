//! Second implementation of mergesort.

use std::fmt::Debug;


fn merge<T: Debug + Clone + PartialOrd>(left: &[T], right: &[T]) -> Vec<T> {
    let mut out = Vec::with_capacity(left.len() + right.len());
    // early return
    if left.last()  < right.first() {
        out.extend_from_slice(left);
        out.extend_from_slice(right);
        return out
    } else if right.last() < left.first() {
        out.extend_from_slice(right);
        out.extend_from_slice(left);
        return out
    }
    let mut xi: usize = 0;
    let mut yi: usize = 0;
    while xi < left.len() && yi < right.len() {
        if left[xi] > right[yi] {
            out.push(right[yi].clone());
            yi += 1;
        } else {
            out.push(left[xi].clone());
            xi += 1;
        }
    }
    out.extend_from_slice(&left[xi..]);
    out.extend_from_slice(&right[yi..]);
    out
}


fn sort<T: Debug + Clone + PartialOrd>(values: &[T]) -> Vec<T> {
     let n_values = values.len();
     if n_values <= 1 {
         return values.to_vec()
     }
     let mid = n_values / 2;
     let left = sort(&values[..mid]);
     let right = sort(&values[mid..]);
     let merged = merge(&left, &right);
     merged
 }


#[cfg(test)]
mod tests {

    #[test]
    fn sort() {
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
            let result = super::sort(case);
            assert_eq!(&result, expect);
        }
    }

}
