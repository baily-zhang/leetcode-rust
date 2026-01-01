use crate::Solution;

// @leet start
impl Solution {
    pub fn xor_operation(n: i32, start: i32) -> i32 {
        (0..n).map(|i| start + i * 2).reduce(|a, b| a ^ b).unwrap()
    }
}
// @leet end
