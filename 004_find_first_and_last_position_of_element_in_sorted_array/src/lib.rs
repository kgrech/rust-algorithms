pub struct Solution {}

fn binary_search_leftmost<T>(arr: &[T], target: &T) -> Option<usize>
where
    T: PartialOrd,
{
    let mut lo = 0usize;
    let mut hi = arr.len();
    while lo < hi {
        let mid = lo + (hi - lo) / 2;
        if *target <= arr[mid] {
            hi = mid // go left
        } else {
            lo = mid + 1 // go right
        }
    }
    if lo < arr.len() && arr[lo] == *target {
        Some(lo)
    } else {
        None
    }
}

fn binary_search_rightmost<T>(arr: &[T], target: &T) -> Option<usize>
where
    T: PartialOrd,
{
    let mut lo = 0usize;
    let mut hi = arr.len();
    while lo < hi {
        let mid = lo + (hi - lo) / 2;
        if *target < arr[mid] {
            hi = mid // go left
        } else {
            lo = mid + 1 // go right
        }
    }
    if hi > 0 && arr[hi - 1] == *target {
        Some(hi - 1)
    } else {
        None
    }
}

impl Solution {
    pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let left = match binary_search_leftmost(&nums, &target) {
            Some(pos) => pos as i32,
            None => -1,
        };
        let right = match binary_search_rightmost(&nums, &target) {
            Some(pos) => pos as i32,
            None => -1,
        };
        vec![left, right]
    }
}
