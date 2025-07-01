use rustc_hash::FxHashMap;
use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn find_lhs(nums: Vec<i32>) -> i32 {
        let mut acc_num: HashMap<i32, usize> = HashMap::new();
        let mut occ: HashMap<i32, bool> = HashMap::new();
        let mut max = 0;
        for &val in nums.iter() {
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

    #[test]
    fn test_find_lhs_optimized() {
        let nums = vec![1, 3, 2, 2, 5, 2, 3, 7];
        assert_eq!(Solution::find_lhs_optimized(nums.clone()), 5);
        assert_eq!(Solution::find_lhs_sort(nums), 5);
    }
}

#[cfg(test)]
mod bench {
    use super::*;
    extern crate test;
    use test::Bencher;

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
    fn bench_optimized_20k(b: &mut Bencher) {
        let base: Vec<i32> = (0..20_000).map(|i| (i as i32 % 1_000) - 500).collect();
        b.iter(|| {
            let nums = test::black_box(base.clone());
            Solution::find_lhs_optimized(nums)
        });
    }
}
