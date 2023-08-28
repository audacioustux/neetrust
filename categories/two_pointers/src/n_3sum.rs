pub struct Solution;

impl Solution {
    pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        use std::{cmp::Ordering::*, collections::HashSet};
        let mut res = HashSet::new();
        let mut nums = nums;
        nums.sort_unstable();

        for (i, num) in nums.iter().enumerate() {
            let target = -num;
            let mut left = i + 1;
            let mut right = nums.len() - 1;

            while left < right {
                let sum = nums[left] + nums[right];

                match sum.cmp(&target) {
                    Less => left += 1,
                    Equal => {
                        res.insert(vec![*num, nums[left], nums[right]]);
                        left += 1;
                        right -= 1;
                    }
                    Greater => right -= 1,
                }
            }
        }
        res.into_iter().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_three_sum() {
        fn is_triplet(nums: &[i32]) -> bool {
            nums.len() == 3 && nums[0] + nums[1] + nums[2] == 0
        }

        Solution::three_sum(vec![-1, 0, 1, 2, -1, -4])
            .iter()
            .for_each(|nums| {
                assert!(is_triplet(nums));
            });
    }
}
