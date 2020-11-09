mod linear;

fn main() {
    println!("Please use cargo test command to run the tests!");
}

#[cfg(test)]
mod tests {
    use crate::linear::Solution;
    use rstest::rstest;

    #[rstest(
    nums,
    expected,
    case(vec![0,1,0,2,1,0,1,3,2,1,2,1], 6)
    )]
    fn when_two_sum_called_then_correct_value_returned(nums: Vec<i32>, expected: i32) {
        assert_eq!(expected, Solution::trap(nums));
    }
}
