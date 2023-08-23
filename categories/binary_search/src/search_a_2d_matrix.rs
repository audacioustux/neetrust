pub struct Solution;

impl Solution {
    pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        match matrix.binary_search_by(|num| num[0].cmp(&target)) {
            Ok(_) => true,
            Err(0) => false,
            Err(i) => {
                let row = &matrix[i - 1];
                row.binary_search(&target).is_ok()
            }
        }
    }
}

#[test]
fn test_search_matrix() {
    assert_eq!(
        Solution::search_matrix(
            vec![vec![1, 3, 5, 7], vec![10, 11, 16, 20], vec![23, 30, 34, 50]],
            3
        ),
        true
    );
    assert_eq!(
        Solution::search_matrix(
            vec![vec![1, 3, 5, 7], vec![10, 11, 16, 20], vec![23, 30, 34, 50]],
            34
        ),
        true
    );
    assert_eq!(
        Solution::search_matrix(
            vec![vec![1, 3, 5, 7], vec![10, 11, 16, 20], vec![23, 30, 34, 50]],
            13
        ),
        false
    );
}
