// LeetCode: Binary Search
// https://leetcode.com/problems/binary-search/

use crate::Solution;

impl Solution {
	pub fn search(nums: Vec<i32>, target: i32) -> i32 {
		let (mut left, mut right) = (0, nums.len());
		
		while left < right {
            let mid = left + (right - left) / 2;
			if nums[mid] < target {
				left = mid + 1;
			} else if nums[mid] > target {
				right = mid;
			} else {
				return mid as i32;
			}
		}
		-1
	}
}