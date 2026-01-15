use crate::Solution;

// @leet start
impl Solution {
    pub fn shuffle(nums: Vec<i32>, n: i32) -> Vec<i32> {
        let n = n as usize;
        (0..n).flat_map(|i| [nums[i], nums[i + n]]).collect()
    }
}
// @leet end
