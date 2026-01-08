use crate::Solution;

// @leet start
impl Solution {
    pub fn is_power_of_three(n: i32) -> bool {
        if n > 0 {
            if n % 3 != 0 {
                false
            } else {
                true
            }
        } else {
            false
        }
    }
}

// @leet end

