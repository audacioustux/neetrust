pub struct Solution;

impl Solution {
    pub fn find_min(nums: Vec<i32>) -> i32 {
        nums[nums.partition_point(|&x| x >= nums[0]) % nums.len()]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_find_min() {
        assert_eq!(Solution::find_min(vec![3, 4, 5, 1, 2]), 1);
        assert_eq!(Solution::find_min(vec![4, 5, 6, 7, 0, 1, 2]), 0);
        assert_eq!(Solution::find_min(vec![11, 13, 15, 17]), 11);
    }
}
