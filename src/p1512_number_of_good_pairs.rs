use crate::Solution;

// @leet start
impl Solution {
    pub fn num_identical_pairs(nums: Vec<i32>) -> i32 {
        nums.iter()
            .enumerate()
            .map(|(i, &val_i)| {
                nums.iter()
                    .skip(i + 1)
                    .filter(|&&val_j| val_i == val_j)
                    .count() as i32
            })
            .sum()
    }
}
// @leet end
