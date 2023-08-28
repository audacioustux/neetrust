pub struct Solution;

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        prices
            .iter()
            .fold((0, i32::MAX), |(max_profit, min_price), &price| {
                (max_profit.max(price - min_price), min_price.min(price))
            })
            .0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_max_profit() {
        assert_eq!(Solution::max_profit(vec![7, 1, 5, 3, 6, 4]), 5);
        assert_eq!(Solution::max_profit(vec![7, 6, 4, 3, 1]), 0);
    }
}
