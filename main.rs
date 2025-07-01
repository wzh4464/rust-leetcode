use rust_leetcode::problems::{
    p3330_find_original_typed_string, p594_longest_harmonious_subsequence,
};

fn main() {
    // Example usage
    let nums = vec![1, 3, 2, 2, 5, 2, 3, 7];
    let result = p594_longest_harmonious_subsequence::Solution::find_lhs(nums);
    println!("Longest harmonious subsequence length: {}", result);

    let word = "abbcccc".to_string();
    let count = p3330_find_original_typed_string::Solution::possible_string_count(word);
    println!("Possible string count: {}", count);
}
