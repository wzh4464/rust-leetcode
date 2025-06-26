/**
 * File: ./rust-test/main.rs
 * Created Date: Tuesday, June 24th 2025
 * Author: Zihan
 * -----
 * Last Modified: Thursday, 26th June 2025 4:19:44 pm
 * Modified By: the developer formerly known as Zihan at <wzh4464@gmail.com>
 * -----
 * HISTORY:
 * Date      		By   	Comments
 * ----------		------	---------------------------------------------------------
**/

struct Solution;

impl Solution {
    pub fn divide_string(s: String, k: i32, fill: char) -> Vec<String> {
        let batch_num = (s.len() - 1) / k as usize + 1;
        let mut result = vec![String::new(); batch_num as usize];
        for i in 0..batch_num {
            result[i] = s[(i * k as usize)..((i + 1) * k as usize).min(s.len())].to_string();
        }
        let last_len = result.last().unwrap().len();
        if last_len < k as usize {
            for _ in last_len..k as usize {
                result.last_mut().unwrap().push(fill);
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_divide_string() {
        let s = "abcdefghi".to_string();
        let k = 3;
        let fill = 'x';
        let result = Solution::divide_string(s, k, fill);
        assert_eq!(result, vec!["abc", "def", "ghi"]);
        
        let s2 = "abcdefghij".to_string();
        let k2 = 3;
        let fill2 = 'x';
        let result2 = Solution::divide_string(s2, k2, fill2);
        assert_eq!(result2, vec!["abc", "def", "ghi", "jxx"]);
    }
}

fn main() {
    todo!()
}
