pub struct Solution;

impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        use std::cmp::Ordering::*;
        let mut left = 0;
        let mut right = nums.len() - 1;
        while left <= right {
            let mid = left + (right - left) / 2;
            match nums[mid].cmp(&target) {
                Equal => return mid as i32,
                Less => left = mid + 1,
                Greater => right = mid - 1,
            }
        }
        -1
    }
}

#[test]
fn test_search() {
    assert_eq!(Solution::search(vec![-1, 0, 3, 5, 9, 12], 9), 4);
    assert_eq!(Solution::search(vec![-1, 0, 3, 5, 9, 12], 2), -1);
}
