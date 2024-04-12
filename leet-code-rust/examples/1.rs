impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        for i in 0..nums.len() {
            for j in i + 1..nums.len() {
                if nums[i] + nums[j] == target {
                    return vec![i as i32, j as i32];
                }
            }
        }
        unreachable!()
    }
}

#[test]
fn case1() {
    assert_eq!(vec![0, 1], Solution::two_sum(vec![2, 7, 11, 15], 9))
}
#[test]
fn case2() {
    assert_eq!(vec![1, 2], Solution::two_sum(vec![3, 2, 4], 6))
}
#[test]
fn case3() {
    assert_eq!(vec![0, 1], Solution::two_sum(vec![3, 3], 6))
}

struct Solution;

fn main() {}
