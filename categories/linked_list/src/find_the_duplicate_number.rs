pub struct Solution;

impl Solution {
    // floyd's tortoise and hare (cycle detection)
    // fast pointer moves twice as fast as slow pointer
    // so fast pointer will meet slow pointer in a cycle
    // assume.
    //      length of cycle = C
    //      start of list ~ start of cycle = P
    //      start of intersection ~ start of cycle = Q
    //      start of cycle ~ intersection = C - Q
    // 2 * slow = fast
    // 2 * (P + C - Q) = P + C + C - Q
    // 2P + 2C - 2Q = P + 2C - Q
    // P = Q
    //
    // [1,2,3,4,5,1]
    //
    // [0] -> [1] -> [2] -> [3]
    //     p   ^             |
    //         | q           |
    //         ------[5] <- [4]
    //
    pub fn find_duplicate(nums: Vec<i32>) -> i32 {
        let mut slow = nums[0];
        let mut fast = nums[0];
        loop {
            slow = nums[slow as usize];
            fast = nums[nums[fast as usize] as usize];
            if slow == fast {
                break;
            }
        }
        let mut slow_at_start = nums[0];
        let mut slow_at_intersection = slow;
        loop {
            if slow_at_start == slow_at_intersection {
                return slow_at_start;
            }
            slow_at_start = nums[slow_at_start as usize];
            slow_at_intersection = nums[slow_at_intersection as usize];
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_duplicate() {
        assert_eq!(Solution::find_duplicate(vec![1, 3, 4, 2, 2]), 2);
        assert_eq!(Solution::find_duplicate(vec![3, 1, 3, 4, 2]), 3);
    }
}
