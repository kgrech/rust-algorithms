use std::collections::HashMap;

pub struct Solution {}

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut seen = HashMap::new();
        for i in 0..nums.len() {
            match seen.get(&(target - nums[i])) {
                Some(j) => return vec![*j as i32, i as i32],
                None => {
                    seen.insert(nums[i], i);
                }
            }
        }
        vec![-1, -1]
    }
}
