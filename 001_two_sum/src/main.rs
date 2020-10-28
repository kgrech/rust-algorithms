mod brute_force;
mod hash_map;

fn main() {
    println!("Please use cargo test command to run the tests!");
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    #[rstest(
    nums,
    target,
    expected,
    case(vec![2,7,11,15], 9, vec![0,1]),
    case(vec![2,7,11,15], 10, vec![-1,-1]),
    case(vec![3,2,4], 6, vec![1,2]),
    )]
    fn when_two_sum_called_then_correct_value_returned(
        nums: Vec<i32>,
        target: i32,
        expected: Vec<i32>,
    ) {
        assert_eq!(
            expected,
            crate::brute_force::Solution::two_sum(nums.to_vec(), target)
        );
        assert_eq!(expected, crate::hash_map::Solution::two_sum(nums, target));
    }
}
