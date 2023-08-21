pub struct Solution;

impl Solution {
    pub fn daily_temperatures(temperatures: Vec<i32>) -> Vec<i32> {
        let mut res = vec![0; temperatures.len()];
        let mut stack = Vec::new();

        for (i, val) in temperatures.iter().enumerate() {
            while !stack.is_empty() && temperatures[*stack.last().unwrap()] < *val {
                let idx = stack.pop().unwrap();
                res[idx] = (i - idx) as i32;
            }
            stack.push(i);
        }
        res
    }
}

#[test]
fn test_daily_temperatures() {
    assert_eq!(
        Solution::daily_temperatures(vec![73, 74, 75, 71, 69, 72, 76, 73]),
        vec![1, 1, 4, 2, 1, 1, 0, 0]
    );
    assert_eq!(
        Solution::daily_temperatures(vec![30, 40, 50, 60]),
        vec![1, 1, 1, 0]
    );
    assert_eq!(
        Solution::daily_temperatures(vec![30, 60, 90]),
        vec![1, 1, 0]
    );
}
