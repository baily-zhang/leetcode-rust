use crate::Solution;

// @leet start
impl Solution {
    pub fn subtract_product_and_sum(n: i32) -> i32 {
        let (sum, product) = n
            .to_string()
            .chars()
            .map(|c| c.to_digit(10).unwrap() as i32)
            .fold((0, 1), |(sum, product), d| (sum + d, product * d));
        product - sum
    }
}
// @leet end
