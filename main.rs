use std::error;

/**
 * File: ./rust-test/main.rs
 * Created Date: Tuesday, June 24th 2025
 * Author: Zihan
 * -----
 * Last Modified: Sunday, 29th June 2025 10:39:08 pm
 * Modified By: the developer formerly known as Zihan at <wzh4464@gmail.com>
 * -----
 * HISTORY:
 * Date      		By   	Comments
 * ----------		------	---------------------------------------------------------
**/

struct Solution;

impl Solution {
    pub fn num_subseq(nums: Vec<i32>, target: i32) -> i32 {
        const MOD: u64 = 1_000_000_007;
        let mut sorted = nums;
        let mut count: u64 = 0;
        // 预先为 acc 分配 nums.len() 的容量，避免后续重复分配
        let mut acc = Vec::with_capacity(sorted.len());
        acc.push(1u64);
        sorted.sort_unstable();
        let mut idx = sorted.len();
        for (i, &num) in sorted.iter().enumerate() {
            idx = sorted[i..idx].partition_point(|&x| x + num <= target) + i;
            if i == idx {
                break;
            }
            let delta = Self::pow_mod(idx as u32 - i as u32 - 1, &mut acc);
            count = (count + delta) % MOD;
        }
        count as i32
    }

    fn pow_mod(x: u32, v: &mut Vec<u64>) -> u64 {
        const MOD: u64 = 1_000_000_007;
        let idx = x as usize;
        if let Some(&result) = v.get(idx) {
            return result;
        }
        let old_len = v.len();
        v.reserve(idx + 1 - old_len);
        for i in old_len..=idx {
            let prev = v[i - 1];
            let curr = prev * 2 % MOD;
            v.push(curr);
        }
        v[idx]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pow_mod() {
        let mut v = vec![1];
        let x = 2;
        let result = Solution::pow_mod( x, &mut v);
        dbg!(&v);
        assert_eq!(result, 4);
    }

    #[test]
    fn test_num_subseq_example_1() {
        let nums = vec![3, 5, 6, 7];
        let target = 9;
        let result = Solution::num_subseq(nums, target);
        assert_eq!(result, 4);
    }

    #[test]
    fn test_num_subseq_example_2() {
        let nums = vec![3, 3, 6, 8];
        let target = 10;
        let result = Solution::num_subseq(nums, target);
        assert_eq!(result, 6);
    }

    #[test]
    fn test_num_subseq_example_3() {
        let nums = vec![2, 3, 3, 4, 6, 7];
        let target = 12;
        let result = Solution::num_subseq(nums, target);
        assert_eq!(result, 61);
    }

    #[test]
    fn test_num_subseq_example_4() {
        let nums = vec![14,4,6,6,20,8,5,6,8,12,6,10,14,9,17,16,9,7,14,11,14,15,13,11,10,18,13,17,17,14,17,7,9,5,10,13,8,5,18,20,7,5,5,15,19,14];
        let target = 22;
        let result = Solution::num_subseq(nums, target);
        assert_eq!(result, 272187084);
    }

    #[test]
    fn test_num_subseq_example_5() {
        let nums = vec![9,25,9,28,24,12,17,8,28,7,21,25,10,2,16,19,12,13,15,28,14,12,24,9,6,7,2,15,19,13,30,30,23,19,11,3,17,2,14,20,22,30,12,1,11,2,2,20,20,27,15,9,10,4,12,30,13,5,2,11,29,5,3,13,22,5,16,19,7,19,11,16,11,25,29,21,29,3,2,9,20,15,9];
        let target = 32;
        let result = Solution::num_subseq(nums, target);
        assert_eq!(result, 91931447);
    }
}

fn main() {
    todo!()
}
