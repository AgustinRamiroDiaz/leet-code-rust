use std::cmp::Reverse;
use std::collections::BinaryHeap;

impl Solution {
    pub fn k_closest(points: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
        let mut points_with_distances: Vec<_> = points
            .iter()
            .map(|point| (point.clone(), point[0].pow(2) + point[1].pow(2)))
            .collect();

        points_with_distances.sort_by(|a, b| a.1.cmp(&b.1));

        points_with_distances
            .into_iter()
            .take(k as usize)
            .map(|p| p.0)
            .collect()
    }
}

#[test]
fn my() {
    let mut h = BinaryHeap::new();

    h.push((1, 1));
    h.push((2, 0));
    h.push((0, 2));

    println!("{h:?}")
}

#[test]
fn case1() {
    assert_eq!(
        vec![vec![-2, 2]],
        Solution::k_closest(vec![vec![1, 3], vec![-2, 2]], 1)
    );
}

struct Solution;

fn main() {}
