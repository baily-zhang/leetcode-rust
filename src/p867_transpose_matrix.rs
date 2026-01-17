use crate::Solution;

// @leet start
impl Solution {
    pub fn transpose(matrix: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let n = matrix[0].len();

        (0..n)
            .map(|j| matrix.iter().map(|row| row[j]).collect())
            .collect()
    }
}
// @leet end
