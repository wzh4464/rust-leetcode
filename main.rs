/**
 * File: ./rust-test/main.rs
 * Created Date: Tuesday, June 24th 2025
 * Author: Zihan
 * -----
 * Last Modified: Saturday, 28th June 2025 9:08:48 pm
 * Modified By: the developer formerly known as Zihan at <wzh4464@gmail.com>
 * -----
 * HISTORY:
 * Date      		By   	Comments
 * ----------		------	---------------------------------------------------------
**/

struct Solution;

impl Solution {
    pub fn max_subsequence(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut sorted_with_idx = nums
            .iter()
            .take(k as usize)
            .cloned()
            .enumerate()
            .collect::<Vec<_>>();
        sorted_with_idx.sort_unstable_by(|a, b| b.1.cmp(&a.1));

        for (i, &num) in nums.iter().skip(k as usize).enumerate() {
            if let Some(&min) = sorted_with_idx.last() {
                let i = i + k as usize;
                if num > min.1 {
                    sorted_with_idx.pop();
                    let idx = sorted_with_idx.partition_point(|&x| x.1 > num);
                    sorted_with_idx.insert(idx, (i, num));
                }
            }
        }
        sorted_with_idx.sort_by_key(|&x| x.0);
        sorted_with_idx.iter().map(|v| v.1).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_subsequence_1() {
        let nums = vec![2, 1, 3, 3];
        let k = 2;
        let result = Solution::max_subsequence(nums, k);
        // 测试将在实现完成后进行验证
        assert_eq!(result, vec![3, 3]);
    }

    #[test]
    fn test_max_subsequence_2() {
        let nums2 = vec![-1, -2, 3, 4];
        let k2 = 3;
        let result2 = Solution::max_subsequence(nums2, k2);
        assert_eq!(result2, vec![-1, 3, 4]);
    }
}

fn main() {
    todo!()
}
