use crate::Solution;

// @leet start
impl Solution {
    pub fn reverse_string(s: &mut Vec<char>) {
        let n = s.len();
        for i in 0..n / 2 {
            // Swap index 'i' with index 'n - 1 - i'
            s.swap(i, n - 1 - i);
        }
    }
}
// @leet end

