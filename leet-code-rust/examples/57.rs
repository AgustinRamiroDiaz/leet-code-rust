impl Solution {
    pub fn insert(intervals: Vec<Vec<i32>>, mut new_interval: Vec<i32>) -> Vec<Vec<i32>> {
        if intervals.is_empty() {
            return vec![new_interval];
        }
        let mut before = Vec::new();
        let mut after = Vec::new();

        let min = new_interval[0];
        let max = new_interval[1];

        for interval in intervals {
            if interval[1] < min {
                before.push(interval)
            } else if interval[0] > max {
                after.push(interval)
            } else {
                new_interval = Self::merge_overlapping(new_interval, interval);
            }
        }

        before.push(new_interval);
        before.append(&mut after);

        before
    }

    fn merge_overlapping(a: Vec<i32>, b: Vec<i32>) -> Vec<i32> {
        vec![a[0].min(b[0]), a[1].max(b[1])]
    }
}

#[test]
fn case1() {
    assert_eq!(
        vec![vec![1, 5], vec![6, 9]],
        Solution::insert(vec![vec![1, 3], vec![6, 9]], vec![2, 5])
    );
}

// #[test]
// fn case2() {
//     assert_eq!(
//         3.0,
//         Solution::find_median_sorted_arrays(vec![], vec![1, 2, 3, 4, 5])
//     );
// }

// #[test]
// fn case3() {
//     assert_eq!(
//         2.0,
//         Solution::find_median_sorted_arrays(vec![1, 3], vec![2])
//     );
// }

// #[test]
// fn case4() {
//     assert_eq!(
//         2.5,
//         Solution::find_median_sorted_arrays(vec![1, 2], vec![3, 4])
//     );
// }

// #[test]
// fn case5() {
//     assert_eq!(
//         0.0,
//         Solution::find_median_sorted_arrays(vec![0, 0], vec![0, 0])
//     );
// }

// #[test]
// fn case6() {
//     assert_eq!(
//         2.5,
//         Solution::find_median_sorted_arrays(vec![-100, 3], vec![2, 400])
//     );
// }

// #[test]
// fn case6() {
//     assert_eq!(
//         3.0,
//         Solution::find_median_sorted_arrays(vec![1, 5], vec![2, 3, 4])
//     );
// }

// #[test]
// fn case7() {
//     assert_eq!(
//         5.0,
//         Solution::find_median_sorted_arrays(vec![1, 3, 5, 5, 5, 5, 70, 90], vec![2, 4, 5, 6, 8])
//     );
// }

// #[test]
// fn case8() {
//     assert_eq!(
//         5.0,
//         Solution::find_median_sorted_arrays(vec![1, 2, 3, 4, 5], vec![6, 7, 8, 9])
//     );
// }

struct Solution;

fn main() {}
