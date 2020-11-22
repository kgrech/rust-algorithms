mod lib;

fn main() {
    println!("Please use cargo test command to run the tests!");
}

#[cfg(test)]
mod tests {
    use crate::lib::Solution;
    use rstest::rstest;

    #[rstest(
    nums,
    target,
    expected,
    case(vec![5,7,7,8,8,10], 8, vec![3,4]),
    case(vec![5,7,7,8,8,10], 6, vec![-1,-1]),
    case(vec![], 0, vec![-1,-1]),
    case(vec![5,7,7,8,8,10], 5, vec![0, 0]),
    case(vec![5,7,7,8,8,10], 10, vec![5, 5]),
    case(vec![5,7,7,8,8,10,10], 10, vec![5, 6]),
    case(vec![5,7,7,8,8,10,10,10], 10, vec![5, 7]),
    case(vec![5], 5, vec![0, 0]),
    case(vec![5], 6, vec![-1, -1]),
    case(vec![5], 4, vec![-1, -1]),
    case(vec![5, 5], 5, vec![0, 1]),
    )]
    fn when_two_sum_called_then_correct_value_returned(
        nums: Vec<i32>,
        target: i32,
        expected: Vec<i32>,
    ) {
        assert_eq!(expected, Solution::search_range(nums, target));
    }
}
