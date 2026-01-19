use crate::Solution;

// @leet start
impl Solution {
    pub fn flip_and_invert_image(image: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        image
            .into_iter()
            .map(|row| row.into_iter().rev().map(|i| 1 ^ i).collect())
            .collect()
    }
}
// @leet end
