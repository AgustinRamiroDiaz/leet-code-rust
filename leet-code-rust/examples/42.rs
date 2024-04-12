use std::collections::VecDeque;

impl Solution {
    pub fn trap(height: Vec<i32>) -> i32 {
        let mut max_to_the_left = vec![];
        let mut max_to_the_right = VecDeque::new();

        let mut current_max = 0;
        max_to_the_left.push(current_max);
        for index in 1..height.len() {
            current_max = current_max.max(height[index - 1]);
            max_to_the_left.push(current_max);
        }

        let mut current_max = 0;
        max_to_the_right.push_front(current_max);
        for index in (0..(height.len() - 1)).rev() {
            current_max = current_max.max(height[index + 1]);
            max_to_the_right.push_front(current_max);
        }

        let mut water = 0;
        for (i, h) in height.iter().enumerate() {
            water += 0.max(max_to_the_left[i].min(max_to_the_right[i]) - h)
        }

        water
    }
}

#[test]
fn case1() {
    assert_eq!(6, Solution::trap(vec![0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1]))
}

#[test]
fn case2() {
    assert_eq!(9, Solution::trap(vec![4, 2, 0, 3, 2, 5]))
}

struct Solution;

fn main() {}
