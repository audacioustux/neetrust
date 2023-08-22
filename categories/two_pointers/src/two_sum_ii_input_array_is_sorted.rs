pub struct Solution;

impl Solution {
    pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
        use std::cmp::Ordering;
        let mut i = 1;
        let mut j = numbers.len();
        while i < j {
            match (numbers[i - 1] + numbers[j - 1]).cmp(&target) {
                Ordering::Less => i += 1,
                Ordering::Equal => return vec![i as i32, j as i32],
                Ordering::Greater => j -= 1,
            }
        }
        vec![]
    }
}

#[test]
fn test_two_sum() {
    assert_eq!(Solution::two_sum(vec![2, 7, 11, 15], 9), vec![1, 2]);
    assert_eq!(Solution::two_sum(vec![2, 3, 4], 6), vec![1, 3]);
    assert_eq!(Solution::two_sum(vec![-1, 0], -1), vec![1, 2]);
}
