pub struct Solution;

impl Solution {
    pub fn largest_rectangle_area(heights: Vec<i32>) -> i32 {
        let mut res = 0;
        let len = heights.len();

        heights
            .iter()
            .enumerate()
            .fold(vec![], |mut stack, (i, height)| {
                let mut start = i;
                while let Some(_) = stack.last().filter(|(_, h)| h > height) {
                    if let Some((idx, h)) = stack.pop() {
                        res = res.max(h * (i - idx) as i32);
                        start = idx;
                    }
                }
                stack.push((start, *height));
                stack
            })
            .iter()
            .for_each(|(idx, h)| {
                res = res.max(h * (len - idx) as i32);
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
