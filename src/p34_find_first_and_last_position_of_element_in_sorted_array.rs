// LeetCode: Find First and Last Position of Element in Sorted Array
// https://leetcode.com/problems/find-first-and-last-position-of-element-in-sorted-array/

use crate::Solution;

impl Solution {
    pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
        match nums.binary_search(&target) {
            Ok(idx) => {
                // 找左边界
                let mut left = idx;
                while left > 0 && nums[left - 1] == target {
                    left -= 1;
                }
                // 找右边界
                let mut right = idx;
                while right + 1 < nums.len() && nums[right + 1] == target {
                    right += 1;
                }
                vec![left as i32, right as i32]
            }
            Err(_) => vec![-1, -1],
        }
    }
}