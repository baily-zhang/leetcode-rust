use crate::Solution;
// @leet start
impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        nums.iter()
            .enumerate()
            .find_map(|(j, &a)| {
                nums.iter()
                    .enumerate()
                    .skip(j + 1)
                    .find(|&(_, &b)| a + b == target)
                    .map(|(i, _)| vec![j as i32, i as i32])
            })
            .unwrap()
    }
}
// @leet end
