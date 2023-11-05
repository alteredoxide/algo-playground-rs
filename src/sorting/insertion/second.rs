//! Second implementation of insertions sort.


fn sort_inplace<T: PartialOrd>(values: &mut [T]) {
    for i in 1..values.len() {
        let mut j = i;
        while j > 0 && values[j-1] > values[j] {
            values.swap(j-1, j);
            j -= 1;
        }
    }
}


fn sort_inplace2<T: PartialOrd>(values: &mut [T]) {
    for i in 1..values.len() {
        for j in (1..=i).rev() {
            if values[j-1] > values[j] { values.swap(j-1, j) }
        }
    }
}


#[cfg(test)]
mod tests {

    #[test]
    fn sort_inplace() {
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
            super::sort_inplace(case);
            assert_eq!(case, expect);
        }
    }

    #[test]
    fn sort_inplace2() {
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
            super::sort_inplace2(case);
            assert_eq!(case, expect);
        }
    }

}
