pub struct Solution;

impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        use std::collections::HashMap;
        let mut map: HashMap<Vec<char>, Vec<String>> = HashMap::new();
        for s in strs {
            let mut chars: Vec<char> = s.chars().collect();
            chars.sort();
            map.entry(chars).or_default().push(s);
        }
        map.values().cloned().collect()
    }
}

#[test]
fn test_group_anagrams() {
    let mut result = Solution::group_anagrams(vec![
        "eat".to_string(),
        "tea".to_string(),
        "tan".to_string(),
        "ate".to_string(),
        "nat".to_string(),
        "bat".to_string(),
    ]);
    let mut expected: Vec<Vec<String>> = vec![
        vec!["ate".to_string(), "eat".to_string(), "tea".to_string()],
        vec!["nat".to_string(), "tan".to_string()],
        vec!["bat".to_string()],
    ];

    result.iter_mut().for_each(|v| v.sort());
    expected.iter_mut().for_each(|v| v.sort());
    result.sort();
    expected.sort();

    assert_eq!(result, expected);
}
