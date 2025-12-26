// LeetCode: Convert the Temperature
// https://leetcode.com/problems/convert-the-temperature/

use crate::Solution;

impl Solution {
    pub fn convert_temperature(celsius: f64) -> Vec<f64> {
        let mut tem = Vec::new();
        tem.push(celsius + 273.15);
        tem.push(celsius * 1.80 + 32.00);
        tem
    }
}