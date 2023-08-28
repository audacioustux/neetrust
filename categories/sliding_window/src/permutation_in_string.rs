pub struct Solution;

impl Solution {
    pub fn check_inclusion(s1: String, s2: String) -> bool {
        if s1.len() > s2.len() {
            return false;
        }

        let mut count = [0; 26];

        s1.bytes().for_each(|b| count[(b - b'a') as usize] += 1);
        s2.bytes()
            .take(s1.len())
            .for_each(|b| count[(b - b'a') as usize] -= 1);

        if count.iter().all(|&c| c == 0) {
            return true;
        }

        s2.bytes().skip(s1.len()).enumerate().any(|(i, b)| {
            count[(s2.as_bytes()[i] - b'a') as usize] += 1;
            count[(b - b'a') as usize] -= 1;

            if count.iter().all(|&c| c == 0) {
                return true;
            }
            false
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_check_inclusion() {
        assert_eq!(
            Solution::check_inclusion("ab".to_string(), "eidbaooo".to_string()),
            true
        );
        assert_eq!(
            Solution::check_inclusion("ab".to_string(), "eidboaoo".to_string()),
            false
        );
        assert_eq!(
            Solution::check_inclusion("ab".to_string(), "a".to_string()),
            false
        );
    }
}
