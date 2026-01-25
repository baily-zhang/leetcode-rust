use crate::Solution;
// @leet start
use std::collections::HashMap;
impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        // Create a HashMap to store numbers we have seen: value -> index
        let mut seen_numbers = HashMap::new();

        for (current_index, &current_num) in nums.iter().enumerate() {
            let complement = target - current_num;

            // Check if the complement (the other number we need) is already in the map
            if let Some(&previous_index) = seen_numbers.get(&complement) {
                // Found it! Return the index of the complement and the current index
                return vec![previous_index as i32, current_index as i32];
            }

            // If not found, store the current number and its index for future checks
            seen_numbers.insert(current_num, current_index);
        }

        vec![] // Should not be reached based on problem constraints
    }
}
// @leet end
