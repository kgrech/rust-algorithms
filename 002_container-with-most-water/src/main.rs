mod brute_force;
mod linear;

fn main() {
    println!("Please use cargo test command to run the tests!");
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    #[rstest(
    height,
    expected,
    case(vec![1,8,6,2,5,4,8,3,7], 49),
    case(vec![1,1], 1),
    case(vec![1,10], 1),
    case(vec![4,3,2,1,4], 16),
    case(vec![1,2,1], 2),
    )]
    fn when_max_area_called_then_correct_value_returned(height: Vec<i32>, expected: i32) {
        assert_eq!(
            expected,
            crate::brute_force::Solution::max_area(height.to_vec())
        );
        assert_eq!(expected, crate::linear::Solution::max_area(height));
    }
}
