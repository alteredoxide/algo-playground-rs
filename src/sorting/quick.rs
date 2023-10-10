//! Implementations of quicksort.

use std::fmt::Debug;
use std::ops::{Add, Div, Sub};

use rand::Rng;


fn select_pivot<T>(values: &[T]) -> (usize, T)
where
    T: Add<Output = T> + Div<Output = T> + Sub<Output = T> + PartialOrd + Copy + From<u8>
{
    if values.len() == 2 {
        return (1, *values.last().unwrap())
    }
    let mut rng = rand::thread_rng();
    let idx = rng.gen_range(0..values.len());
    (idx, values[idx])
}


pub fn sort<T>(values: &mut [T])
where
    T: Add<Output = T> + Div<Output = T> + Sub<Output = T> + From<u8> + Copy + Debug + PartialEq + PartialOrd
{
    if values.len() < 2 {
        return
    }
    let (mut idx, pivot) = select_pivot(values);
    //dbg!(&pivot);
    for i in 0..values.len() {
        if i == idx {
            continue
        }
        //dbg!(i, idx);
        match values[i] {
            v if v > pivot && i < idx => {
                values.swap(i, idx);
                idx = i;
            },
            v if v < pivot => {
                if i > idx + 1 {
                    values.swap(idx, idx+1);
                    values.swap(idx, i);
                    idx += 1;
                } else if i == idx + 1 {
                    values.swap(idx, i);
                    idx = i;
                }
            },
            v if v == pivot => {
                let mut j = i;
                if i > idx {
                    while j > idx {
                        values.swap(j, j-1);
                        j -= 1;
                    }
                }
            },
            _ => {},
        }
        //dbg!(&values);
    }
    sort(&mut values[0..idx]);
    sort(&mut values[idx+1..]);
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
            super::sort(case);
            assert_eq!(case, expect);
        }
    }
}
