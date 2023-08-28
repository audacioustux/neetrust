pub struct Solution;

impl Solution {
    pub fn character_replacement(s: String, k: i32) -> i32 {
        use std::collections::HashMap;
        let chars = s.chars().collect::<Vec<char>>();
        let mut map: HashMap<char, usize> = HashMap::new();
        let mut res = 0;
        let mut left = 0;
        for (right, c) in chars.iter().enumerate() {
            *map.entry(*c).or_insert(0) += 1;
            if (right - left + 1) as i32 - *map.values().max().unwrap() as i32 > k {
                *map.get_mut(&chars[left]).unwrap() -= 1;
                left += 1;
            }
            res = res.max(right - left + 1);
        }
        res as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_character_replacement() {
        assert_eq!(Solution::character_replacement("ABAB".to_string(), 2), 4);
        assert_eq!(Solution::character_replacement("AABABBA".to_string(), 1), 4);
    }
}
