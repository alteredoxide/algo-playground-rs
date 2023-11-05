//! An implementation of bubble sort


fn sort<T>(values: &mut [T])
where
    T: PartialEq + PartialOrd
{
    let n = values.len();
    let mut swapped: bool;
    for i in 0..n-1 {
        swapped = false;
        for j in (i+1..n).rev() {
            if &values[j] < &values[j-1] {
                values.swap(j, j-1);
                swapped = true;
            }
        }
        if !swapped {
            // early exit
            break
        }
    }
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
            super::sort(case);
            assert_eq!(case, expect);
        }
    }
}
