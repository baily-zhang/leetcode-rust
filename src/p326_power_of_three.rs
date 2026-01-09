use crate::Solution;

// @leet start
impl Solution {
    pub fn is_power_of_three(n: i32) -> bool {
        let mut n = n;
        if n > 0 {
            while n % 3 == 0 {
                n /= 3;
            }
        }
        n == 1
    }
}
// @leet end
