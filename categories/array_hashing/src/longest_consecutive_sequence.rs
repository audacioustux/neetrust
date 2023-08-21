pub struct Solution;

impl Solution {
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        use std::collections::HashSet;
        let nums: HashSet<i32> = HashSet::from_iter(nums.into_iter());

        let mut res = 0;
        for num in &nums {
            if !nums.contains(&(num - 1)) {
                let mut cur = num + 1;
                while nums.contains(&cur) {
                    cur += 1;
                }
                res = res.max(cur - num);
            }
        }
        res
    }
}

#[test]
fn test_longest_consecutive() {
    assert_eq!(Solution::longest_consecutive(vec![100, 4, 200, 1, 3, 2]), 4);
    assert_eq!(
        Solution::longest_consecutive(vec![0, 3, 7, 2, 5, 8, 4, 6, 0, 1]),
        9
    );
}
