use crate::Solution;

// @leet start
impl Solution {
    pub fn transpose(matrix: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let row = matrix[0].len();
        (0..row)
            .map(|j| matrix.iter().map(|row| row[j]).collect())
            .collect()
    }
}
// @leet end
