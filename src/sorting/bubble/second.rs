//! Second pass of bubble sort.


fn sort<T: PartialOrd>(values: &mut [T]) {
    let mut modified = true;
    while modified {
        modified = false;
        for i in 0..(values.len() - 1) {
            if values[i] > values[i+1] {
                values.swap(i, i+1);
                modified = true;
            }
        }
    }
}


fn swap_elements<T>(values: &mut [T], i: usize, j: usize) {
    let pa = std::ptr::addr_of_mut!(values[i]);
    let pb = std::ptr::addr_of_mut!(values[j]);
    unsafe {
        std::ptr::swap(pa, pb);
    }
}


fn swap_elements_offset<T>(values: &mut [T], i: usize, j: usize) {
    if i >= values.len() || j >= values.len() {
        panic!("index out of bounds");
    }
    let ptr = values.as_mut_ptr();
    unsafe {
        let pa = ptr.offset(i as isize);
        let pb = ptr.offset(j as isize);
        std::ptr::swap(pa, pb);
    }
}


fn sort_ptr<T: PartialOrd>(values: &mut [T]) {
    let mut modified = true;
    while modified {
        modified = false;
        for i in 0..(values.len() - 1) {
            if values[i] > values[i+1] {
                swap_elements_offset(values, i, i+1);
                modified = true;
            }
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

    #[test]
    fn sort_ptr() {
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
            super::sort_ptr(case);
            assert_eq!(case, expect);
        }
    }
}
