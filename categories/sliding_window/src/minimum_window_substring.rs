pub struct Solution;

impl Solution {
    pub fn min_window(s: String, t: String) -> String {
        let s = s.as_bytes();
        let t = t.as_bytes();
        if s.len() < t.len() {
            return "".to_string();
        }
        let mut window = [0; 128];
        let (mut have, need) = (
            0,
            t.iter().fold([0; 128], |mut acc, b| {
                acc[*b as usize] += 1;
                acc
            }),
        );
        let mut left = 0;
        let mut res: Option<&[u8]> = None;
        for (right, &b) in s.iter().enumerate() {
            window[b as usize] += 1;
            if window[b as usize] <= need[b as usize] {
                have += 1;
            }
            while have == t.len() {
                if res.is_none() || res.as_ref().unwrap().len() > right - left + 1 {
                    res = Some(&s[left..=right]);
                }
                window[s[left] as usize] -= 1;
                if window[s[left] as usize] < need[s[left] as usize] {
                    have -= 1;
                }
                left += 1;
            }
        }
        res.map_or("".to_string(), |s| String::from_utf8(s.to_vec()).unwrap())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_min_window() {
        assert_eq!(
            Solution::min_window("ADOBECODEBANC".to_string(), "ABC".to_string()),
            "BANC".to_string()
        );
        assert_eq!(
            Solution::min_window("a".to_string(), "a".to_string()),
            "a".to_string()
        );
        assert_eq!(
            Solution::min_window("a".to_string(), "aa".to_string()),
            "".to_string()
        );
        assert_eq!(
            Solution::min_window("ab".to_string(), "a".to_string()),
            "a".to_string()
        );
        assert_eq!(
            Solution::min_window("a".to_string(), "b".to_string()),
            "".to_string()
        );
    }
}
