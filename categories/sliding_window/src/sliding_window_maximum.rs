pub struct Solution;

impl Solution {
    pub fn max_sliding_window(nums: Vec<i32>, k: i32) -> Vec<i32> {
        use std::collections::VecDeque;
        let mut result = Vec::new();
        let mut deque = VecDeque::new();
        for i in 0..nums.len() {
            while !deque.is_empty() && nums[*deque.back().unwrap()] < nums[i] {
                deque.pop_back();
            }
            deque.push_back(i);
            if i >= k as usize - 1 {
                result.push(nums[*deque.front().unwrap()]);
                if *deque.front().unwrap() == i + 1 - k as usize {
                    deque.pop_front();
                }
            }
        }
        result
    }
}
