pub struct Solution;

impl Solution {
    pub fn encode(strs: Vec<String>) -> String {
        let mut res = String::new();
        for s in strs {
            res.push_str(&format!("{}#{}", s.len(), s));
        }
        res
    }

    pub fn decode(s: String) -> Vec<String> {
        let mut res = vec![];
        let mut iter = s.chars();

        while let Some(c) = iter.next() {
            let mut len: usize = 0;
            while c.is_digit(10) {
                len = len * 10 + c.to_digit(10).unwrap() as usize;
                if let Some('#') = iter.next() {
                    break;
                }
            }
             
            res.push(iter.by_ref().take(len).collect());
        }

        res
    }
}

#[test]
fn test_encode_and_decode_strings() {
    assert_eq!(
        Solution::decode(Solution::encode(vec![
            "Hello".to_string(),
            "World".to_string()
        ])),
        vec!["Hello".to_string(), "World".to_string()]
    );
    // assert_eq!(
    //     Solution::decode(Solution::encode(vec![
    //         "5#Hello".to_string(),
    //         "World5".to_string()
    //     ])),
    //     vec!["5#Hello".to_string(), "World5".to_string()]
    // );
}
