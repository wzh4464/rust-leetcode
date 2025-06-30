#![feature(test)]

#[cfg(test)]
extern crate test;
use std::{i32};

use rustc_hash::FxHashMap;
/**
 * File: ./rust-test/main.rs
 * Created Date: Tuesday, June 24th 2025
 * Author: Zihan
 * -----
 * Last Modified: Monday, 30th June 2025 12:30:28 pm
 * Modified By: the developer formerly known as Zihan at <wzh4464@gmail.com>
 * -----
 * HISTORY:
 * Date      		By   	Comments
 * ----------		------	---------------------------------------------------------
**/
use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn find_lhs(nums: Vec<i32>) -> i32 {
        // let mut hash: HashMap<usize> = nums.iter().enumerate().map(|(i, &x)| (i, x)).collect();
        // let acc_num = Vec::<usize>::with_capacity(WHOLE_LEN);
        // let mut acc_num = vec![0usize; WHOLE_LEN];
        let mut acc_num: HashMap<i32, usize> = HashMap::new();
        let mut occ: HashMap<i32, bool> = HashMap::new();
        let mut max = 0;
        for &val in nums.iter() {
            // acc_num[val as usize + HALF_LEN + 2] += 1;
            *acc_num.entry(val + 2).or_insert(0) += 1;
            *acc_num.entry(val + 1).or_insert(0) += 1;
            occ.insert(val, true);

            // 检查 (val, val+1)
            if occ.get(&val) == Some(&true) && occ.get(&(val + 1)) == Some(&true) {
                if let Some(&cnt) = acc_num.get(&(val + 2)) {
                    max = max.max(cnt);
                }
            }
            // 检查 (val-1, val)
            if occ.get(&val) == Some(&true) && occ.get(&(val - 1)) == Some(&true) {
                if let Some(&cnt) = acc_num.get(&(val + 1)) {
                    max = max.max(cnt);
                }
            }
        }

        max as i32
    }

    /// Optimized HashMap version with FxHashMap and single-table logic
    pub fn find_lhs_optimized(nums: Vec<i32>) -> i32 {
        if nums.len() < 2 {
            return 0;
        }

        let mut freq: FxHashMap<i32, usize> =
            FxHashMap::with_capacity_and_hasher(nums.len(), Default::default());

        for &val in &nums {
            *freq.entry(val).or_insert(0) += 1;
        }

        let mut max = 0;
        for (&key, &count) in &freq {
            if let Some(&next_count) = freq.get(&(key + 1)) {
                max = max.max(count + next_count);
            }
        }

        max as i32
    }

    /// Sorting + two‑pointer version: in‑place, cache‑friendly.
    pub fn find_lhs_sort(mut nums: Vec<i32>) -> i32 {
        if nums.len() < 2 {
            return 0;
        }
        nums.sort_unstable();
        let mut start = 0usize;
        let mut max = 0usize;
        for end in 0..nums.len() {
            while nums[end] - nums[start] > 1 {
                start += 1;
            }
            if nums[end] - nums[start] == 1 {
                max = max.max(end - start + 1);
            }
        }
        max as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn test_find_lhs_example_1() {
        let nums = vec![1, 3, 2, 2, 5, 2, 3, 7];
        let result = Solution::find_lhs(nums);
        assert_eq!(result, 5);
    }

    #[test]
    fn test_find_lhs_example_2() {
        let nums = vec![1, 2, 3, 4];
        let result = Solution::find_lhs(nums);
        assert_eq!(result, 2);
    }

    #[test]
    fn test_find_lhs_example_3() {
        let nums = vec![1, 1, 1, 1];
        let result = Solution::find_lhs(nums);
        assert_eq!(result, 0);
    }

    #[test]
    fn test_find_lhs_empty() {
        let nums = vec![];
        let result = Solution::find_lhs(nums);
        assert_eq!(result, 0);
    }

    #[test]
    fn test_find_lhs_single_element() {
        let nums = vec![1];
        let result = Solution::find_lhs(nums);
        assert_eq!(result, 0);
    }

    #[bench]
    fn bench_hash_20k(b: &mut Bencher) {
        let base: Vec<i32> = (0..20_000).map(|i| (i as i32 % 1_000) - 500).collect();
        b.iter(|| {
            let nums = test::black_box(base.clone());
            Solution::find_lhs(nums)
        });
    }

    #[bench]
    fn bench_sort_20k(b: &mut Bencher) {
        let base: Vec<i32> = (0..20_000).map(|i| (i as i32 % 1_000) - 500).collect();
        b.iter(|| {
            let nums = test::black_box(base.clone());
            Solution::find_lhs_sort(nums)
        });
    }

    #[bench]
    fn bench_hash_large_sparse_100k(b: &mut Bencher) {
        let base: Vec<i32> = (0..100_000).step_by(100).map(|i| i as i32).collect();
        b.iter(|| {
            let nums = test::black_box(base.clone());
            Solution::find_lhs(nums)
        });
    }

    #[bench]
    fn bench_sort_large_sparse_100k(b: &mut Bencher) {
        let base: Vec<i32> = (0..100_000).step_by(100).map(|i| i as i32).collect();
        b.iter(|| {
            let nums = test::black_box(base.clone());
            Solution::find_lhs_sort(nums)
        });
    }

    #[bench]
    fn bench_hash_large_sparse_1m(b: &mut Bencher) {
        let base: Vec<i32> = (0..1_000_000).step_by(1000).map(|i| i as i32).collect();
        b.iter(|| {
            let nums = test::black_box(base.clone());
            Solution::find_lhs(nums)
        });
    }

    #[bench]
    fn bench_sort_large_sparse_1m(b: &mut Bencher) {
        let base: Vec<i32> = (0..1_000_000).step_by(1000).map(|i| i as i32).collect();
        b.iter(|| {
            let nums = test::black_box(base.clone());
            Solution::find_lhs_sort(nums)
        });
    }

    #[bench]
    fn bench_hash_very_sparse_10m(b: &mut Bencher) {
        let base: Vec<i32> = (0..10_000_000).step_by(50000).map(|i| i as i32).collect();
        b.iter(|| {
            let nums = test::black_box(base.clone());
            Solution::find_lhs(nums)
        });
    }

    #[bench]
    fn bench_sort_very_sparse_10m(b: &mut Bencher) {
        let base: Vec<i32> = (0..10_000_000).step_by(50000).map(|i| i as i32).collect();
        b.iter(|| {
            let nums = test::black_box(base.clone());
            Solution::find_lhs_sort(nums)
        });
    }

    #[bench]
    fn bench_optimized_20k(b: &mut Bencher) {
        let base: Vec<i32> = (0..20_000).map(|i| (i as i32 % 1_000) - 500).collect();
        b.iter(|| {
            let nums = test::black_box(base.clone());
            Solution::find_lhs_optimized(nums)
        });
    }

    #[bench]
    fn bench_optimized_200k(b: &mut Bencher) {
        let base: Vec<i32> = (0..200_000).map(|i| (i as i32 % 10_000) - 5000).collect();
        b.iter(|| {
            let nums = test::black_box(base.clone());
            Solution::find_lhs_optimized(nums)
        });
    }

    #[bench]
    fn bench_sort_200k(b: &mut Bencher) {
        let base: Vec<i32> = (0..200_000).map(|i| (i as i32 % 10_000) - 5000).collect();
        b.iter(|| {
            let nums = test::black_box(base.clone());
            Solution::find_lhs_sort(nums)
        });
    }

    #[bench]
    fn bench_optimized_1m(b: &mut Bencher) {
        let base: Vec<i32> = (0..1_000_000)
            .map(|i| (i as i32 % 50_000) - 25000)
            .collect();
        b.iter(|| {
            let nums = test::black_box(base.clone());
            Solution::find_lhs_optimized(nums)
        });
    }

    #[bench]
    fn bench_sort_1m(b: &mut Bencher) {
        let base: Vec<i32> = (0..1_000_000)
            .map(|i| (i as i32 % 50_000) - 25000)
            .collect();
        b.iter(|| {
            let nums = test::black_box(base.clone());
            Solution::find_lhs_sort(nums)
        });
    }
}

fn main() {
    todo!()
}
