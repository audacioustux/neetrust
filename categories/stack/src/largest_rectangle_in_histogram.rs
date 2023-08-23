pub struct Solution;

impl Solution {
    pub fn largest_rectangle_area(heights: Vec<i32>) -> i32 {
        use std::iter::once;
        let mut res = 0;

        heights
            .iter()
            .chain(once(&0))
            .enumerate()
            .fold(vec![], |mut stack, (i, height)| {
                let mut start = i;
                while stack.last().filter(|(_, h)| h > height).is_some() {
                    if let Some((idx, h)) = stack.pop() {
                        res = res.max(h * (i - idx) as i32);
                        start = idx;
                    }
                }
                stack.push((start, *height));
                stack
            });

        res
    }
}

#[test]
fn test_largest_rectangle_area() {
    assert_eq!(Solution::largest_rectangle_area(vec![2, 1, 5, 6, 2, 3]), 10);
    assert_eq!(Solution::largest_rectangle_area(vec![2, 4]), 4);
    assert_eq!(Solution::largest_rectangle_area(vec![2, 1, 2]), 3);
    assert_eq!(
        Solution::largest_rectangle_area(vec![1, 1, 1, 1, 1, 1, 1, 1]),
        8
    );
    assert_eq!(
        Solution::largest_rectangle_area(vec![1, 1, 1, 1, 1, 10, 1, 1]),
        10
    );
}
