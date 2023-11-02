//! Second pass at binary search, just to practice and to explore different
//! implementations.

use std::{cmp::Ordering, fmt::Debug};


fn binary_recurse<T>(values: &[T], x: &T, offset: usize) -> Result<Option<usize>, String>
where
    T: Ord + PartialOrd
{
    if values.is_empty() { return Ok(None) }
    let n_values = values.len();
    let mid = n_values / 2;
    let x_mid = &values[mid];
    match x.partial_cmp(x_mid) {
        Some(Ordering::Equal) => Ok(Some(mid + offset)),
        Some(Ordering::Less) => binary_recurse(&values[..mid], x, 0),
        Some(Ordering::Greater) => binary_recurse(&values[mid+1..], x, mid+1),
        None => Err(String::from("unable to compare values"))
    }
}


/// Perform binary search on a slice of monotincially increasing values
/// and return an optional index of the first occurence.
fn binary_search<T>(values: &[T], x: &T) -> Result<Option<usize>, String>
where
    T: Debug + Ord + PartialOrd
{
    binary_recurse(values, x, 0)
}


#[cfg(test)]
mod tests {

    #[test]
    fn binary_search() {
        let data = vec![1_i32, 3, 4, 5, 7, 1, 15, 19];
        let test_cases = vec![
            (1, Ok(Some(0))),
            (3, Ok(Some(1))),
            (15, Ok(Some(6))),
            (13, Ok(None)),
        ];
        for (value, expected) in test_cases {
            let result = super::binary_search(&data, &value);
            assert_eq!(result, expected);
        }
    }
}
