pub struct Solution;

impl Solution {
    /// 3330. Find the Original Typed String I
    ///
    /// Alice tends to press a key for too long, resulting in a character being typed multiple times.
    /// She may have done this at most once.
    /// Return the total number of possible original strings that Alice might have intended to type.
    ///
    /// # Examples
    ///
    /// ```
    /// use rust_leetcode::problems::p3330_find_original_typed_string::Solution;
    /// assert_eq!(Solution::possible_string_count("abbcccc".to_string()), 5);
    /// assert_eq!(Solution::possible_string_count("abcd".to_string()), 1);
    /// assert_eq!(Solution::possible_string_count("aaaa".to_string()), 4);
    /// ```
    pub fn possible_string_count(word: String) -> i32 {
        Self::possible_string_count_iterative(word)
    }

    pub fn possible_string_count_iterative(word: String) -> i32 {
        let mut current_char: Option<char> = None;
        let mut current_int = 0;
        let mut acc = 1;

        for ch in (&word).chars() {
            if let Some(cur_char) = current_char {
                if cur_char == ch {
                    current_int += 1;
                } else {
                    acc += current_int;
                    current_int = 0;
                    current_char = Some(ch);
                }
            } else {
                current_char = Some(ch);
            }
        }

        acc + current_int
    }

    pub fn possible_string_count_functional(word: String) -> i32 {
        let (_, result) = word.chars().fold((None, 1), |(prev, acc), ch| match prev {
            None => (Some(ch), acc),
            Some(p) => {
                if p == ch {
                    (prev, acc + 1)
                } else {
                    (Some(ch), acc)
                }
            }
        });
        result
    }

    pub fn possible_string_count_vector(word: String) -> i32 {
        let mut ans = 1;
        let chars: Vec<char> = word.chars().collect();
        for i in 1..chars.len() {
            if chars[i - 1] == chars[i] {
                ans += 1;
            }
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_possible_string_count_example_1() {
        let word = "abbcccc".to_string();
        let result = Solution::possible_string_count(word);
        assert_eq!(result, 5);
    }

    #[test]
    fn test_possible_string_count_example_2() {
        let word = "abcd".to_string();
        let result = Solution::possible_string_count(word);
        assert_eq!(result, 1);
    }

    #[test]
    fn test_possible_string_count_example_3() {
        let word = "aaaa".to_string();
        let result = Solution::possible_string_count(word);
        assert_eq!(result, 4);
    }

    #[test]
    fn test_possible_string_count_single_char() {
        let word = "a".to_string();
        let result = Solution::possible_string_count(word);
        assert_eq!(result, 1);
    }

    #[test]
    fn test_possible_string_count_no_consecutive() {
        let word = "abcdef".to_string();
        let result = Solution::possible_string_count(word);
        assert_eq!(result, 1);
    }

    #[test]
    fn test_possible_string_count_all_same() {
        let word = "aaaaa".to_string();
        let result = Solution::possible_string_count(word);
        assert_eq!(result, 5);
    }

    #[test]
    fn test_possible_string_count_two_consecutive_groups() {
        let word = "aabbcc".to_string();
        let result = Solution::possible_string_count(word);
        assert_eq!(result, 4);
    }

    #[test]
    fn test_possible_string_count_empty() {
        let word = "".to_string();
        let result = Solution::possible_string_count(word);
        assert_eq!(result, 1);
    }
}

#[cfg(test)]
mod bench {
    use super::*;
    extern crate test;
    use test::Bencher;

    #[bench]
    fn bench_iterative_short(b: &mut Bencher) {
        let word = "abbcccc".to_string();
        b.iter(|| {
            let w = test::black_box(word.clone());
            Solution::possible_string_count_iterative(w)
        });
    }

    #[bench]
    fn bench_functional_short(b: &mut Bencher) {
        let word = "abbcccc".to_string();
        b.iter(|| {
            let w = test::black_box(word.clone());
            Solution::possible_string_count_functional(w)
        });
    }

    #[bench]
    fn bench_iterative_medium(b: &mut Bencher) {
        let word = "a".repeat(100) + &"b".repeat(100) + &"c".repeat(100);
        b.iter(|| {
            let w = test::black_box(word.clone());
            Solution::possible_string_count_iterative(w)
        });
    }

    #[bench]
    fn bench_functional_medium(b: &mut Bencher) {
        let word = "a".repeat(100) + &"b".repeat(100) + &"c".repeat(100);
        b.iter(|| {
            let w = test::black_box(word.clone());
            Solution::possible_string_count_functional(w)
        });
    }

    #[bench]
    fn bench_iterative_long(b: &mut Bencher) {
        let word = "a".repeat(1000) + &"b".repeat(1000) + &"c".repeat(1000);
        b.iter(|| {
            let w = test::black_box(word.clone());
            Solution::possible_string_count_iterative(w)
        });
    }

    #[bench]
    fn bench_functional_long(b: &mut Bencher) {
        let word = "a".repeat(1000) + &"b".repeat(1000) + &"c".repeat(1000);
        b.iter(|| {
            let w = test::black_box(word.clone());
            Solution::possible_string_count_functional(w)
        });
    }

    #[bench]
    fn bench_iterative_very_long(b: &mut Bencher) {
        let word = "a".repeat(10000) + &"b".repeat(10000) + &"c".repeat(10000);
        b.iter(|| {
            let w = test::black_box(word.clone());
            Solution::possible_string_count_iterative(w)
        });
    }

    #[bench]
    fn bench_functional_very_long(b: &mut Bencher) {
        let word = "a".repeat(10000) + &"b".repeat(10000) + &"c".repeat(10000);
        b.iter(|| {
            let w = test::black_box(word.clone());
            Solution::possible_string_count_functional(w)
        });
    }

    #[bench]
    fn bench_iterative_alternating(b: &mut Bencher) {
        let mut word = String::with_capacity(1000);
        for i in 0..500 {
            word.push_str(&format!("{}", (i % 26 + 97) as u8 as char));
            word.push_str(&format!("{}", (i % 26 + 97) as u8 as char));
        }
        b.iter(|| {
            let w = test::black_box(word.clone());
            Solution::possible_string_count_iterative(w)
        });
    }

    #[bench]
    fn bench_functional_alternating(b: &mut Bencher) {
        let mut word = String::with_capacity(1000);
        for i in 0..500 {
            word.push_str(&format!("{}", (i % 26 + 97) as u8 as char));
            word.push_str(&format!("{}", (i % 26 + 97) as u8 as char));
        }
        b.iter(|| {
            let w = test::black_box(word.clone());
            Solution::possible_string_count_functional(w)
        });
    }

    #[bench]
    fn bench_vector_short(b: &mut Bencher) {
        let word = "abbcccc".to_string();
        b.iter(|| {
            let w = test::black_box(word.clone());
            Solution::possible_string_count_vector(w)
        });
    }

    #[bench]
    fn bench_vector_medium(b: &mut Bencher) {
        let word = "a".repeat(100) + &"b".repeat(100) + &"c".repeat(100);
        b.iter(|| {
            let w = test::black_box(word.clone());
            Solution::possible_string_count_vector(w)
        });
    }

    #[bench]
    fn bench_vector_long(b: &mut Bencher) {
        let word = "a".repeat(1000) + &"b".repeat(1000) + &"c".repeat(1000);
        b.iter(|| {
            let w = test::black_box(word.clone());
            Solution::possible_string_count_vector(w)
        });
    }

    #[bench]
    fn bench_vector_very_long(b: &mut Bencher) {
        let word = "a".repeat(10000) + &"b".repeat(10000) + &"c".repeat(10000);
        b.iter(|| {
            let w = test::black_box(word.clone());
            Solution::possible_string_count_vector(w)
        });
    }

    #[bench]
    fn bench_vector_alternating(b: &mut Bencher) {
        let mut word = String::with_capacity(1000);
        for i in 0..500 {
            word.push_str(&format!("{}", (i % 26 + 97) as u8 as char));
            word.push_str(&format!("{}", (i % 26 + 97) as u8 as char));
        }
        b.iter(|| {
            let w = test::black_box(word.clone());
            Solution::possible_string_count_vector(w)
        });
    }
}
