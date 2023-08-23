pub struct Solution;

impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let pivot = nums.partition_point(|&x| x >= nums[0]);

        match nums[..pivot].binary_search(&target) {
            Ok(i) => i as i32,
            Err(_) => match nums[pivot..].binary_search(&target) {
                Ok(i) => (pivot + i) as i32,
                Err(_) => -1,
            },
        }
    }
}

#[test]
fn test_search() {
    assert_eq!(Solution::search(vec![4, 5, 6, 7, 0, 1, 2], 0), 4);
    assert_eq!(Solution::search(vec![4, 5, 6, 7, 0, 1, 2], 3), -1);
    assert_eq!(Solution::search(vec![1], 0), -1);
    assert_eq!(Solution::search(vec![1], 1), 0);
    assert_eq!(Solution::search(vec![1, 3], 3), 1);
    assert_eq!(Solution::search(vec![3, 1], 3), 0);
    assert_eq!(Solution::search(vec![5, 1, 3], 3), 2);
}
