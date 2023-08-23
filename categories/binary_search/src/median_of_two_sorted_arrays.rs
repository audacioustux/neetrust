pub struct Solution;

impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let len = nums1.len() + nums2.len();
        let mut sorted = Vec::with_capacity(len / 2);

        let (mut i, mut j) = (0, 0);
        while i < nums1.len() && j < nums2.len() && i + j <= len / 2 {
            if nums1[i] < nums2[j] {
                sorted.push(nums1[i]);
                i += 1;
            } else {
                sorted.push(nums2[j]);
                j += 1;
            }
        }
        let (i, j) = (i.min(nums1.len()), j.min(nums2.len()));
        sorted.extend_from_slice(&nums1[i..]);
        sorted.extend_from_slice(&nums2[j..]);

        if len % 2 == 0 {
            (sorted[len / 2 - 1] + sorted[len / 2]) as f64 / 2.0
        } else {
            sorted[len / 2] as f64
        }
    }
}

#[test]
fn test_find_median_sorted_arrays() {
    assert_eq!(
        Solution::find_median_sorted_arrays(vec![1, 3], vec![2]),
        2.0
    );
    assert_eq!(
        Solution::find_median_sorted_arrays(vec![1, 2], vec![3, 4]),
        2.5
    );
}
