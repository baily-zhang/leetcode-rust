use crate::Solution;

// @leet start
impl Solution {
    pub fn transpose(matrix: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let n = matrix[0].len();
        (0..n)
            .map(|i| matrix.iter().map(|row| row[i]).collect())
            .collect()
    }
}
// @leet end
