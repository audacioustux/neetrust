pub struct Solution;

impl Solution {
    pub fn trap(height: Vec<i32>) -> i32 {
        let (mut left_max, mut right_max) = (0, 0);
        let mut res = 0;

        let mut left = 0;
        let mut right = height.len() - 1;
        while left < right {
            if height[left] < height[right] {
                left_max = left_max.max(height[left]);
                res += left_max - height[left];
                left += 1;
            } else {
                right_max = right_max.max(height[right]);
                res += right_max - height[right];
                right -= 1;
            }
        }

        res
    }
}

#[test]
fn test_trap() {
    assert_eq!(Solution::trap(vec![0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1]), 6);
    assert_eq!(Solution::trap(vec![4, 2, 0, 3, 2, 5]), 9);
}
