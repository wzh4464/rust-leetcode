pub struct Solution;

impl Solution {
    pub fn kth_character(k: i32) -> char {
        let mut binary = format!("{:b}", k);
        let mut count = 0u8;
        while binary.len() > 1 {
            match binary.chars().last() {
                Some('1') => {
                    binary = binary.chars().skip(1).collect();
                    match binary.chars().position(|c| c == '1') {
                        None => (),
                        Some(idx) => {
                            let (_, binary_str) = binary.split_at(idx);
                            binary = binary_str.to_string();
                        }
                    }
                    count += 1;
                }
                Some('0') => {
                    binary.pop();
                    count += 1
                }
                _ => {
                    println!("Not happening")
                }
            }
        }
        // binary.chars()
        ('a' as u8 + count) as char
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let k = 5;
        assert_eq!(Solution::kth_character(k), 'b');
    }

    #[test]
    fn test_example_2() {
        let k = 10;
        assert_eq!(Solution::kth_character(k), 'c');
    }
}
