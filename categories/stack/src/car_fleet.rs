pub struct Solution;

impl Solution {
    pub fn car_fleet(target: i32, position: Vec<i32>, speed: Vec<i32>) -> i32 {
        let mut cars: Vec<(i32, f64)> = position
            .into_iter()
            .zip(speed)
            .map(|(p, s)| (p, (target - p) as f64 / s as f64))
            .collect();
        cars.sort_by(|(p1, _), (p2, _)| p2.cmp(p1));

        let mut res = 0;
        let mut cur = 0.0;
        for (_, time) in cars {
            if time > cur {
                res += 1;
                cur = time;
            }
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_car_fleet() {
        assert_eq!(
            Solution::car_fleet(12, vec![10, 8, 0, 5, 3], vec![2, 4, 1, 1, 3]),
            3
        );
        assert_eq!(Solution::car_fleet(10, vec![3], vec![3]), 1);
        assert_eq!(Solution::car_fleet(10, vec![0, 4, 2], vec![2, 1, 3]), 1);
    }
}
