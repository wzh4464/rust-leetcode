/**
 * File: ./rust-test/main.rs
 * Created Date: Tuesday, June 24th 2025
 * Author: Zihan
 * -----
 * Last Modified: Thursday, 26th June 2025 3:47:46 pm
 * Modified By: the developer formerly known as Zihan at <wzh4464@gmail.com>
 * -----
 * HISTORY:
 * Date      		By   	Comments
 * ----------		------	---------------------------------------------------------
**/

struct Solution;

impl Solution {
    pub fn longest_subsequence(s: String, k: i32) -> i32 {
        // let mut acc = 0;
        // let mut temp = 0;

        let k_binary = Self::k_to_binary(k);
        let k_len = k_binary.len();
        let s_part = &s[s.len().saturating_sub(k_len) ..];
        match Self::binary_to_decimal(s_part) {
            Some(ds) => {
                if ds <= k {
                    return (s[..s.len().saturating_sub(k_len)].chars().filter(|&c| c == '0').count() as i32
                        + k_len as i32).min(s.len() as i32);
                } else {
                    return s[..s.len().saturating_sub(k_len)].chars().filter(|&c| c == '0').count() as i32
                        + k_len as i32
                        - 1;
                }
            }
            _ => {
                panic!(
                    "Invalid input: the last {} bits of '{}' do not form a valid binary number less than or equal to {}",
                    k_len, s, k
                );
            }
        }
    }

    fn k_to_binary(k: i32) -> String {
        format!("{:b}", k)
    }

    fn binary_to_decimal(s: &str) -> Option<i32> {
        i32::from_str_radix(s, 2).ok()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_k_to_binary() {
        assert_eq!(Solution::k_to_binary(0), "0");
        assert_eq!(Solution::k_to_binary(1), "1");
        assert_eq!(Solution::k_to_binary(2), "10");
        assert_eq!(Solution::k_to_binary(3), "11");
        assert_eq!(Solution::k_to_binary(4), "100");
        assert_eq!(Solution::k_to_binary(5), "101");
        assert_eq!(Solution::k_to_binary(6), "110");
    }

    #[test]
    fn test_binary_to_decimal() {
        assert_eq!(Solution::binary_to_decimal("0").unwrap(), 0);
        assert_eq!(Solution::binary_to_decimal("1").unwrap(), 1);
        assert_eq!(Solution::binary_to_decimal("10").unwrap(), 2);
        assert_eq!(Solution::binary_to_decimal("11").unwrap(), 3);
        assert_eq!(Solution::binary_to_decimal("100").unwrap(), 4);
        assert_eq!(Solution::binary_to_decimal("101").unwrap(), 5);
    }

    #[test]
    fn test_longest_subsequence() {
        assert_eq!(Solution::longest_subsequence("1001010".to_string(), 5), 5);
    }

    #[test]
    fn test_longest_subsequence_2() {
        assert_eq!(Solution::longest_subsequence("0".to_string(), 583196182), 1);
    }
}

fn main() {
    todo!()
}
