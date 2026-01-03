use crate::Solution;

// @leet start
impl Solution {
    pub fn add_digits(num: i32) -> i32 {
        let mut n = num;
        while n >= 10 {
            n = n
                .to_string()
                .chars()
                .fold(0, |acc, a| acc + a.to_digit(10).unwrap() as i32)
        }
        n
    }
}
// @leet end

