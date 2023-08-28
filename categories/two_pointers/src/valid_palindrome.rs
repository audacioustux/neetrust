pub struct Solution;

impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        let mut chars = s.chars().filter(char::is_ascii_alphanumeric);
        while let (Some(a), Some(b)) = (chars.next(), chars.next_back()) {
            if !a.eq_ignore_ascii_case(&b) {
                return false;
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_valid_palindrome() {
        assert_eq!(
            Solution::is_palindrome("A man, a plan, a canal: Panama".to_string()),
            true
        );
        assert_eq!(Solution::is_palindrome("race a car".to_string()), false);
        assert_eq!(Solution::is_palindrome(" ".to_string()), true);
    }
}
