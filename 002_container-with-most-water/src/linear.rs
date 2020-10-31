use std::cmp::max;

pub struct Solution {}

impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let mut lo = 0;
        let mut hi = height.len() - 1;
        let mut max_area = std::i32::MIN;

        while lo < hi {
            if height[lo] > height[hi] {
                max_area = max(max_area, (hi - lo) as i32 * height[hi]);
                hi -= 1;
            } else {
                max_area = max(max_area, (hi - lo) as i32 * height[lo]);
                lo += 1;
            }
        }
        max_area
    }
}
