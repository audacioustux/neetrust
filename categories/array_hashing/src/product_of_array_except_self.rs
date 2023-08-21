pub struct Solution;

impl Solution {
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        let mut res = vec![1; nums.len()];
        let mut left = 1;
        let mut right = 1;
        for i in 0..nums.len() {
            res[i] *= left;
            res[nums.len() - 1 - i] *= right;
            left *= nums[i];
            right *= nums[nums.len() - 1 - i];
        }
        res
    }
}

#[test]
fn test_product_except_self() {
    assert_eq!(
        Solution::product_except_self(vec![1, 2, 3, 4]),
        vec![24, 12, 8, 6]
    );
    assert_eq!(
        Solution::product_except_self(vec![-1, 1, 0, -3, 3]),
        vec![0, 0, 9, 0, 0]
    );
}
