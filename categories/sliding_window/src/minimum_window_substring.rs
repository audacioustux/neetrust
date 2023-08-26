pub struct Solution;

impl Solution {
    pub fn min_window(s: String, t: String) -> String {
        use std::collections::HashMap;
        let mut map = HashMap::new();
        t.bytes().for_each(|b| *map.entry(b).or_insert(0) += 1);

        let mut res: Option<&[u8]> = None;
        let mut left = 0;
        let s = s.into_bytes();

        for (end, &end_char) in s.iter().enumerate() {
            map.entry(end_char).and_modify(|n| *n -= 1);
            if res.is_some() || map.values().all(|n| *n <= 0) {
                while map.get(&s[left]).copied().unwrap_or(-1) < 0 {
                    map.entry(s[left]).and_modify(|n| *n += 1);
                    left += 1;
                }
                if res.is_none() || res.unwrap().len() > end - left {
                    res = Some(&s[left..=end]);
                }
            }
        }
        String::from_utf8(res.unwrap_or_default().to_vec()).unwrap()
    }
}

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
}
