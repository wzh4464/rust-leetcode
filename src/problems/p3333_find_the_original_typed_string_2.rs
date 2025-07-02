pub struct Solution;

const MOD: i32 = 1_000_000_007;

impl Solution {
    /// [Problem Number]. [Problem Title]
    ///
    /// [Problem Description]
    ///
    /// # Examples
    ///
    /// ```
    /// use rust_leetcode::problems::[module_name]::Solution;
    /// assert_eq!(Solution::[method_name]([example_input]), [expected_output]);
    /// ```
    pub fn possible_string_count(word: String, k: i32) -> i32 {
        let (_, acc_vec) = word.chars().fold(
            (None, Vec::<i32>::new()),
            |(prev, mut acc_vec), ch| match prev {
                Some(p) if p == ch => (prev, {
                    *(acc_vec.last_mut().unwrap()) += 1;
                    acc_vec
                }),
                _ => (Some(ch), {
                    acc_vec.push(0);
                    acc_vec
                }),
            },
        );

        let await_to_distribute = word.len() as i32 - k;

        let m = acc_vec.len();

        let mut dp: Vec<Vec<Option<i32>>> = vec![vec![None; await_to_distribute as usize + 1]; m];

        (0..=await_to_distribute).fold(0, |acc, j| {
            (acc + Solution::drawer(m - 1, j as usize, &mut dp, &acc_vec)) % MOD
        })

        // todo!("Implement the solution")
    }

    fn drawer(m: usize, k: usize, dp: &mut Vec<Vec<Option<i32>>>, acc_vec: &Vec<i32>) -> i32 {
        let compute = |dp: &mut Vec<Vec<Option<i32>>>| {
            let prev_max_delete = acc_vec[0..m].iter().fold(0, |acc, &x| acc + x);
            let iter = k.min(prev_max_delete as usize);
            let acc = (0..=iter).fold(0, |exact_kind, i| {
                let j = k - i; // now delete
                if j <= acc_vec[m] as usize {
                    (exact_kind + Solution::drawer(m - 1, i, dp, acc_vec)) % MOD
                } else {
                    exact_kind
                }
            });
            dp[m][k] = Some(acc);
            acc
        };

        match dp[m][k] {
            Some(i) => i,
            None if m * k == 0 => {
                dp[m][k] = Some(1);
                1
            }
            _ => compute(dp),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        // Test case 1
        let word = "aabbccdd";
        let k = 7;
        assert_eq!(Solution::possible_string_count(word.to_string(), k), 5);
    }

    #[test]
    fn test_example_2() {
        // Test case 2
        let word = "aaabbb";
        let k = 3;
        assert_eq!(Solution::possible_string_count(word.to_string(), k), 8);
    }

    #[test]
    fn test_edge_case_empty() {
        // Edge case: empty input
        todo!("Add edge case test")
    }

    #[test]
    fn test_edge_case_single() {
        // Edge case: single element
        todo!("Add edge case test")
    }
}

#[cfg(test)]
mod bench {
    use super::*;
    extern crate test;
    use test::Bencher;

    #[bench]
    fn bench_small_input(b: &mut Bencher) {
        // Small input benchmark
        todo!("Add benchmark")
    }

    #[bench]
    fn bench_medium_input(b: &mut Bencher) {
        // Medium input benchmark
        todo!("Add benchmark")
    }

    #[bench]
    fn bench_large_input(b: &mut Bencher) {
        // Large input benchmark
        todo!("Add benchmark")
    }
}
