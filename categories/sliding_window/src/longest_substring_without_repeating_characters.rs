pub struct Solution;

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        use std::collections::HashMap;
        let mut map: HashMap<char, usize> = HashMap::new();
        let mut res = 0;
        let mut left = 0;
        for (right, c) in s.chars().enumerate() {
            if let Some(&idx) = map.get(&c) {
                left = left.max(idx + 1);
            }
            res = res.max(right - left + 1);
            map.insert(c, right);
        }
        res as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_length_of_longest_substring() {
        assert_eq!(
            Solution::length_of_longest_substring("abcabcbb".to_string()),
            3
        );
        assert_eq!(
            Solution::length_of_longest_substring("bbbbb".to_string()),
            1
        );
        assert_eq!(
            Solution::length_of_longest_substring("pwwkew".to_string()),
            3
        );
    }
}
