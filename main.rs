/**
 * File: ./rust-test/main.rs
 * Created Date: Tuesday, June 24th 2025
 * Author: Zihan
 * -----
 * Last Modified: Tuesday, 24th June 2025 10:33:56 am
 * Modified By: the developer formerly known as Zihan at <wzh4464@gmail.com>
 * -----
 * HISTORY:
 * Date      		By   	Comments
 * ----------		------	---------------------------------------------------------
**/
use std::{
    io::{self, Write},
    thread::current,
};

fn input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");
    input.trim().to_string()
}

struct Solution;

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        const LENGTH:usize = 256;
        let mut alphabet: [usize; LENGTH] = [usize::MIN; LENGTH];
        let mut new_init = usize::MIN;
        let mut max_length = 0;
        for (i, c) in s.chars().enumerate() {
            let index = (c as u8 - b'a') as usize;
            let current_length = i + 1 - alphabet[index].max(new_init);
            if alphabet[index] > new_init {
                new_init = alphabet[index];
            }
            alphabet[index] = i + 1;

            if current_length > max_length {
                max_length = current_length;
            }
            // println!("{:?}", alphabet);
            // println!("new_init:{:}", new_init);
        }

        max_length as i32
    }
}

fn main() {
    // 手动输入测试用例
    println!("Testing longest substring without repeating characters:");

    // Example: input
    let test1 = input("Enter a string: ").to_string();
    let result1 = Solution::length_of_longest_substring(test1.clone());
    println!("Input: '{}', Output: {}", test1, result1);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let result = Solution::length_of_longest_substring("abcabcbb".to_string());
        assert_eq!(result, 3);
    }

    #[test]
    fn test_example_2() {
        let result = Solution::length_of_longest_substring("bbbbb".to_string());
        assert_eq!(result, 1);
    }

    #[test]
    fn test_example_3() {
        let result = Solution::length_of_longest_substring("pwwkew".to_string());
        assert_eq!(result, 3);
    }

    #[test]
    fn test_empty_string() {
        let result = Solution::length_of_longest_substring("".to_string());
        assert_eq!(result, 0);
    }

    #[test]
    fn test_single_character() {
        let result = Solution::length_of_longest_substring("a".to_string());
        assert_eq!(result, 1);
    }

    #[test]
    fn test_all_unique() {
        let result = Solution::length_of_longest_substring("abcdef".to_string());
        assert_eq!(result, 6);
    }
}
