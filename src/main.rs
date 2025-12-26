use leetcodes::Solution;

fn main() {
    // Test convert_the_temperature_2469
    let result = Solution::convert_temperature(36.50);
    println!("convert_temperature(36.50) = {:?}", result);

    // Test binary_search_704
    let nums = vec![-1, 0, 3, 5, 9, 12];
    let result = Solution::search(nums, 9);
    println!("search([-1,0,3,5,9,12], 9) = {}", result);
}
