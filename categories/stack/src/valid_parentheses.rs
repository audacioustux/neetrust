pub struct Solution;

impl Solution {
    pub fn is_valid(s: String) -> bool {
        use std::collections::HashMap;
        let mut map = HashMap::new();
        map.insert(')', '(');
        map.insert('}', '{');
        map.insert(']', '[');

        let mut stack: Vec<char> = Vec::new();
        for c in s.chars() {
            if map.contains_key(&c) {
                if stack.pop() != map.get(&c).copied() {
                    return false;
                }
            } else {
                stack.push(c);
            }
        }
        stack.is_empty()
    }
}

#[test]
fn test_is_valid() {
    assert_eq!(Solution::is_valid("()".to_string()), true);
    assert_eq!(Solution::is_valid("()[]{}".to_string()), true);
    assert_eq!(Solution::is_valid("(]".to_string()), false);
    assert_eq!(Solution::is_valid("([)]".to_string()), false);
    assert_eq!(Solution::is_valid("{[]}".to_string()), true);
}
