// LeetCode: Smallest Even Multiple
// https://leetcode.com/problems/smallest-even-multiple/

use crate::Solution;

impl Solution {
    pub fn smallest_even_multiple(n: i32) -> i32 {
        match n {
            n if n % 2 == 0 => n,
            n => 2 * n,
        }
    }
}