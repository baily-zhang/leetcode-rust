use crate::Solution;

// @leet start
impl Solution {
    pub fn flip_and_invert_image(image: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        image
            .into_iter() // 1. Manager: "Here is a conveyor belt of Rows."
            .map(|row| {
                // 2. "For each Row..."
                row.into_iter() // 3. Worker: "Unpack the pixels."
                    .rev() // 4. "Reverse the pixels (Horizontal Flip)."
                    .map(|x| 1 ^ x) // 5. "Invert the pixel."
                    .collect() // 6. "Repackage into a new Row."
            })
            .collect() // 7. Manager: "Stack all the new Rows."
    }
}
// @leet end

