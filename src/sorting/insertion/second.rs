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


fn sort_inplace3<T: PartialOrd>(values: &mut [T]) {
    for i in 1..values.len() {
        for j in 0..i {
            if values[i] < values[j] {
                values.swap(i, j);
            }
        }
    }
}


/// Manual implementation of swapping slice elements using pointers of the elements.
/// NOTE: This function uses unsafe code, but still meets guaranteeds. The arg type for
/// `values` could be an immutable ref if this function were itself labeled as unsafe,
/// thereby pushing responsibility onto the caller to guarantee correctness.
fn swap_elements<T: PartialOrd>(values: &mut [T], i: usize, j: usize) {
    let pa = std::ptr::addr_of_mut!(values[i]);
    let pb = std::ptr::addr_of_mut!(values[j]);
    unsafe {
        std::ptr::swap(pa, pb);
    }
}


/// Manual implementation of swapping slice elements using slice pointer with offset.
/// NOTE: Uses unsafe code; similar to `swap_elements()`.
fn swap_elements_offset<T: PartialOrd>(values: &mut [T], i: usize, j: usize) {
    if i >= values.len() || j >= values.len() {
        panic!("index out of bounds");
    }
    unsafe {
        let pa = values.as_ptr().offset(i as isize) as *mut T;
        let pb = values.as_ptr().offset(j as isize) as *mut T;
        std::ptr::swap(pa, pb);
    }
}


fn sort_inplace_ptrs<T: PartialOrd>(values: &mut [T]) {
    for i in 1..values.len() {
        for j in (1..=i).rev() {
            if values[j-1] > values[j] { swap_elements_offset(values, j-1, j) }
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
            super::sort_inplace3(case);
            assert_eq!(case, expect);
        }
    }

    #[test]
    fn sort_inplace_ptrs() {
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
            super::sort_inplace_ptrs(case);
            assert_eq!(case, expect);
        }
    }

}
