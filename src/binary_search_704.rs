use crate::Solution;

impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        match nums.binary_search(&target) {
            Ok(v) => v as i32,
            Err(_) => -1,
        }
    }
}
