use crate::Solution;
// @leet start
use std::collections::HashMap;
impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map = HashMap::with_capacity(nums.len()); // 预先分配足够的容量

        for (i, &num) in nums.iter().enumerate() {
            let complement = target - num;
            if let Some(&index) = map.get(&complement) {
                return vec![index, i as i32]; // 如果 index 已经是 i32 类型，可以省略转换
            }
            map.insert(num, i as i32); // 直接存储为 i32 类型
        }

        unreachable!() // 根据 LeetCode 的约束，总是有解的
    }
}

// @leet end

