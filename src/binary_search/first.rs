//! A random implementation of binary search on a `Vec`.


pub fn binary_search<T>(data: &[T], value: &T) -> Option<usize>
    where
        T: PartialOrd + PartialEq
{
    let mut max_idx = data.len();
    let mut min_idx = 0;
    let mut mid = (max_idx + 1) / 2;
    let mut result = &data[mid];
    if result == value {
        return Some(mid)
    }
    let mut i = 0;
    while result != value && i < 10 {
        i += 1;
        mid = match result > value {
            true => {
                max_idx = mid;
                match mid {
                    1 => 0,
                    _ => (min_idx + mid + 1) / 2
                }
            },
            false => {
                min_idx = mid;
                (mid + max_idx + 1) / 2
            }
        };
        result = &data[mid];
    }
    if result == value {
        return Some(mid)
    }
    None
}


#[cfg(test)]
mod test {

    #[test]
    fn binary_search_i32() {
        let data = vec![1_i32, 3, 4, 5, 7, 10, 15, 19];
        let test_cases = vec![
            (1, Some(0)),
            (3, Some(1)),
            (15, Some(6)),
            (13, None),
        ];
        for (value, expected) in test_cases {
            let result = super::binary_search(&data, &value);
            assert_eq!(result, expected);
        }
    }
}
