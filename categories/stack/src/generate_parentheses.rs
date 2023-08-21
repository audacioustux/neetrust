pub struct Solution;

impl Solution {
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        let mut res = Vec::new();

        fn backtrack(res: &mut Vec<String>, cur: String, open: i32, close: i32, max: i32) {
            if cur.len() == (max * 2) as usize {
                res.push(cur);
                return;
            }

            if open < max {
                backtrack(res, format!("{}(", cur), open + 1, close, max);
            }
            if close < open {
                backtrack(res, format!("{})", cur), open, close + 1, max);
            }
        }

        backtrack(&mut res, "".into(), 0, 0, n);
        res
    }
}

#[test]
fn test_generate_parenthesis() {
    assert_eq!(
        Solution::generate_parenthesis(3),
        vec![
            "((()))".to_string(),
            "(()())".to_string(),
            "(())()".to_string(),
            "()(())".to_string(),
            "()()()".to_string()
        ]
    );
}
