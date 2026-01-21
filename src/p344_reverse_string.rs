use crate::Solution;

// @leet start
impl Solution {
    pub fn reverse_string(s: &mut Vec<char>) {
        let len = s.len();
        if len < 2 {
            return;
        }

        // 1. The Split
        let (left_half, right_half) = s.split_at_mut(len / 2);

        // 2. The Iterator Chain
        left_half
            .iter_mut()
            .zip(right_half.iter_mut().rev())
            .for_each(|(a, b)| {
                std::mem::swap(a, b);
            });
    }
}
// @leet end
