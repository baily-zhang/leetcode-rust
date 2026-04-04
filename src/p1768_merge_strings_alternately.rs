use crate::Solution;

// @leet start
impl Solution {
    pub fn merge_alternately(word1: String, word2: String) -> String {
        let mut a = word1.chars();
        let mut b = word2.chars();

        let mut out = String::new();

        loop {
            match (a.next(), b.next()) {
                (Some(c1), Some(c2)) => {
                    out.push(c1);
                    out.push(c2);
                }
                (Some(c1), None) => {
                    out.push(c1);
                    out.extend(a);
                    break;
                }
                (None, Some(c2)) => {
                    out.push(c2);
                    out.extend(b);
                    break;
                }
                (None, None) => break,
            }
        }
        out
    }
}
// @leet end
