use std::cmp::{max, min};

pub struct Solution {}

impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let mut max_area = std::i32::MIN;
        for i in 0..height.len() {
            for j in i + 1..height.len() {
                let len = j - i;
                let h = min(height[i], height[j]);
                max_area = max(max_area, len as i32 * h);
            }
        }
        max_area
    }
}
