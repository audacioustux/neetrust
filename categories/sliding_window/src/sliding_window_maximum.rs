pub struct Solution;

impl Solution {
    pub fn max_sliding_window(nums: Vec<i32>, k: i32) -> Vec<i32> {
        use std::collections::VecDeque;
        let k = k as usize;
        let mut res = Vec::with_capacity(nums.len() - k + 1);
        let mut deque = VecDeque::with_capacity(k);

        let mut left = 0;
        for (right, &num) in nums.iter().enumerate() {
            // remove out of range
            if let Some(&idx) = deque.front() {
                if idx < left {
                    deque.pop_front();
                }
            }
            // remove smaller
            while deque.back().map_or(false, |&idx| nums[idx] < num) {
                deque.pop_back();
            }
            // push current
            deque.push_back(right);

            let window_size = right - left + 1;
            // if window is full
            if window_size == k {
                res.push(nums[deque.front().unwrap().clone()]); // max is the first
                left += 1; // move left cursor to right
            }
        }
        res
    }
}

#[test]
fn test_max_sliding_window() {
    assert_eq!(
        Solution::max_sliding_window(vec![1, 3, -1, -3, 5, 3, 6, 7], 3),
        vec![3, 3, 5, 5, 6, 7]
    );
    assert_eq!(Solution::max_sliding_window(vec![1, -1], 1), vec![1, -1]);
}
