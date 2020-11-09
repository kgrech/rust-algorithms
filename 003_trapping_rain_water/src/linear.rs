use std::cmp::{max, min};

pub struct Solution {}

impl Solution {
    pub fn trap(height: Vec<i32>) -> i32 {
        if height.is_empty() {
            return 0;
        }

        let mut max_left = vec![height[0]; height.len()];
        for i in 1..height.len() {
            max_left[i] = max(max_left[i - 1], height[i]);
        }

        let mut max_right = vec![height[height.len() - 1]; height.len()];
        for i in (0..height.len() - 1).rev() {
            max_right[i] = max(max_right[i + 1], height[i]);
        }

        let mut sum = 0;
        for i in 0..height.len() {
            sum += min(max_left[i], max_right[i]) - height[i];
        }
        sum
    }
}
