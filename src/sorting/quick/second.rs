//! Second implementation of quicksort.

use std::{cmp::Ordering, fmt::Debug};

use rand::Rng;


fn get_pivot_idx(n_values: usize) -> usize {
    let mut rng = rand::thread_rng();
    rng.gen_range(0..n_values)
}


fn sort_inplace<T: Debug + PartialOrd>(values: &mut [T]) {
    if values.len() < 2 { return }
    let mut pivot = get_pivot_idx(values.len());
    let mut i = 0;
    while i < values.len() {
        match i.cmp(&pivot) {
            Ordering::Equal => { i += 1 },
            Ordering::Less => {
                if values[i] > values[pivot] {
                    values.swap(pivot, pivot-1);
                    if i < pivot - 1{
                        values.swap(pivot, i);
                    }
                    pivot -= 1;
                } else {
                    i += 1;
                }
            },
            Ordering::Greater => {
                if values[i] < values[pivot] {
                    values.swap(pivot, pivot + 1);
                    if i > pivot + 1 {
                        values.swap(pivot, i);
                        i += 1;
                    }
                    pivot += 1;
                } else {
                    i += 1;
                }
            },
        }
        //dbg!(&values);
    }
    sort_inplace(&mut values[..pivot]);
    sort_inplace(&mut values[pivot+1..]);
}


#[cfg(test)]
mod tests {
    #[test]
    fn sort() {
        let mut cases: Vec<Vec<i64>> = vec![
            vec![0, 2, 1, 4, 7, 3],
            vec![0, 2, 1, 4, 7, 0],
            vec![2, -1, 5, 2, 9],
            vec![0, 1, 2, 4, 3, 0],
            vec![3, 7, 8, 5, 2, 1, 9, 5, 4],
        ];
        let expected: Vec<Vec<i64>> = vec![
            vec![0, 1, 2, 3, 4, 7],
            vec![0, 0, 1, 2, 4, 7],
            vec![-1, 2, 2, 5, 9],
            vec![0, 0, 1, 2, 3, 4],
            vec![1, 2, 3, 4, 5, 5, 7, 8, 9],
        ];
        for (case, expect) in cases.iter_mut().zip(&expected) {
            super::sort_inplace::<i64>(case);
            assert_eq!(case, expect);
        }
    }
}
