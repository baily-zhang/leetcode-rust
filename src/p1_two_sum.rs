use crate::Solution;
// @leet start
impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        nums.iter().enumerate().find_map(|(i, &a)| {
            nums.iter()
                .enumerate()
                .skip(i + 1)
                .find(|(_, &b)| a + b == target)
                .map()
        })
    }
}
// @leet end
