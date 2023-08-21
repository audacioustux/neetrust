pub struct Solution;

impl Solution {
    pub fn largest_rectangle_area(heights: Vec<i32>) -> i32 {
        let mut res = 0;
        let mut stack = Vec::new();

        for (i, &height) in heights.iter().chain([0].iter()).enumerate() {
            while let Some(&top) = stack.last() {
                if heights[top] < height {
                    break;
                }
                stack.pop();
                let left = if stack.is_empty() { -1 } else { stack.last().copied().unwrap() as i32 };
                res = res.max(heights[top] * (i as i32 - left - 1));
            }
            stack.push(i);
        }
        res
    }
}

#[test]
fn test_largest_rectangle_area() {
    assert_eq!(Solution::largest_rectangle_area(vec![2, 1, 5, 6, 2, 3]), 10);
    assert_eq!(Solution::largest_rectangle_area(vec![2, 4]), 4);
    assert_eq!(Solution::largest_rectangle_area(vec![2, 1, 2]), 3);
    assert_eq!(Solution::largest_rectangle_area(vec![1, 1, 1, 1, 1, 1, 1, 1]), 8);
    assert_eq!(Solution::largest_rectangle_area(vec![1, 1, 1, 1, 1, 10, 1, 1]), 10);
}