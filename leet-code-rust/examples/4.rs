impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let nums1_range = (0, nums1.len());
        let nums2_range = (0, nums2.len());

        let nums1_median_index = (nums1_range.0 + nums1_range.1) / 2;
        let mut nums1_median = nums1[nums1_median_index];

        let nums2_median_index = (nums2_range.0 + nums2_range.1) / 2;
        let mut nums2_median = nums2[nums2_median_index];

        if nums1_median < nums2_median {
            if nums1[nums1_median_index + 1] > nums2_median
                && nums1_median > nums2[nums2_median_index - 1]
            {
                return (nums1_median + nums2_median) as f64 / 2.0;
            } else {
            }
        } else {
            todo!()
        }
        todo!()
    }
}

struct Solution;

#[test]
fn case1() {
    assert_eq!(
        3.0,
        Solution::find_median_sorted_arrays(vec![1, 2, 3, 4, 5], vec![])
    );
}

#[test]
fn case2() {
    assert_eq!(
        3.0,
        Solution::find_median_sorted_arrays(vec![], vec![1, 2, 3, 4, 5])
    );
}

#[test]
fn case3() {
    assert_eq!(
        2.0,
        Solution::find_median_sorted_arrays(vec![1, 3], vec![2])
    );
}

#[test]
fn case4() {
    assert_eq!(
        2.5,
        Solution::find_median_sorted_arrays(vec![1, 2], vec![3, 4])
    );
}

#[test]
fn case5() {
    assert_eq!(
        0.0,
        Solution::find_median_sorted_arrays(vec![0, 0], vec![0, 0])
    );
}

#[test]
fn case6() {
    assert_eq!(
        2.5,
        Solution::find_median_sorted_arrays(vec![-100, 3], vec![2, 400])
    );
}

#[test]
fn case6() {
    assert_eq!(
        3.0,
        Solution::find_median_sorted_arrays(vec![1, 5], vec![2, 3, 4])
    );
}

#[test]
fn case7() {
    assert_eq!(
        5.0,
        Solution::find_median_sorted_arrays(vec![1, 3, 5, 5, 5, 5, 70, 90], vec![2, 4, 5, 6, 8])
    );
}

#[test]
fn case8() {
    assert_eq!(
        5.0,
        Solution::find_median_sorted_arrays(vec![1, 2, 3, 4, 5], vec![6, 7, 8, 9])
    );
}

fn main() {}
