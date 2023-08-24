pub struct Solution;

impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        if nums1.len() > nums2.len() {
            return Solution::find_median_sorted_arrays(nums2, nums1);
        }

        let (len1, len2) = (nums1.len(), nums2.len());
        let total_len = len1 + len2;
        let (mut left, mut right) = (0, len1);
        loop {
            let mid1 = left + (right - left) / 2;
            let mid2 = (total_len + 1) / 2 - mid1;
            let (l1, r1, l2, r2) = (
                if mid1 == 0 { i32::MIN } else { nums1[mid1 - 1] },
                if mid1 == len1 { i32::MAX } else { nums1[mid1] },
                if mid2 == 0 { i32::MIN } else { nums2[mid2 - 1] },
                if mid2 == len2 { i32::MAX } else { nums2[mid2] },
            );
            if l1 <= r2 && l2 <= r1 {
                if total_len % 2 == 0 {
                    return (l1.max(l2) as f64 + r1.min(r2) as f64) / 2.0;
                } else {
                    return l1.max(l2) as f64;
                }
            } else if l1 > r2 {
                right = mid1 - 1;
            } else {
                left = mid1 + 1;
            }
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
