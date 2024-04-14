impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let mut from = 0;
        let mut to = nums.len();

        while from != to {
            if to - from == 1 {
                if nums[from] == target {
                    return from as i32;
                } else {
                    return -1;
                }
            }

            let half = (to + from) / 2;

            let middle = nums[half];

            if middle == target {
                return half as i32;
            }

            if middle < target {
                from = half;
            } else {
                to = half;
            }
        }

        -1
    }
}

#[test]
fn case1() {
    assert_eq!(4, Solution::search(vec![-1, 0, 3, 5, 9, 12], 9))
}
#[test]
fn case2() {
    assert_eq!(-1, Solution::search(vec![-1, 0, 3, 5, 9, 12], 2))
}
// #[test]
// fn case3() {
//     assert_eq!(vec![0, 1], Solution::two_sum(vec![3, 3], 6))
// }

struct Solution;

fn main() {}
