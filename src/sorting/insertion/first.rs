//! A strictly in-place implementation of insertion sort.

pub fn sort<T>(values: &mut [T])
    where
        T: PartialEq + PartialOrd
{
    let n = values.len();
    for i in 1..n {
        let mut k = i;
        while k > 0 && values[k] < values[k-1] {
            values.swap(k, k-1);
            k -= 1;
        }
    }
}


/// Instead of using the swap method, directly implement that swap.
/// NOTE: the swap is only duplicating what the implementation of the `swap()` method
/// available on `Vec`. Doing this in only for the purpose of exploring and better
/// understanding what's happening at a lower level.
pub fn sort_ptrs<T>(values: &mut [T])
    where
        T: PartialEq + PartialOrd
{
    let n = values.len();
    for i in 1..n {
        let mut k = i;
        while k > 0 && values[k] < values[k-1] {
            // get two raw pointers: one to each value in memory
            let pa = std::ptr::addr_of_mut!(values[k]);
            let pb = std::ptr::addr_of_mut!(values[k-1]);
            unsafe {
                std::ptr::swap(pa, pb);
            }
            k -= 1;
        }
    }
}


/// Instead of using the swap method, directly and completely implement the swap.
/// NOTE: this goes beyond the macro used in the `sort_prts` implementation.
pub fn sort_ptrs_manual<T>(values: &mut [T])
    where
        T: PartialEq + PartialOrd
{
    let n = values.len();
    for i in 1..n {
        let mut k = i;
        while k > 0 && values[k] < values[k-1] {
            // get two raw pointers: one to each value in memory
            unsafe {
                let pa = values.as_ptr().offset(k as isize) as *mut T;
                let pb = values.as_ptr().offset((k-1) as isize) as *mut T;
                std::ptr::swap(pa, pb);
            }
            k -= 1;
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
    fn sort_ptrs() {
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
            super::sort_ptrs(case);
            assert_eq!(case, expect);
        }
    }

    #[test]
    fn sort_ptrs_manual() {
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
            super::sort_ptrs_manual(case);
            assert_eq!(case, expect);
        }
    }
}
