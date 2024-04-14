use std::collections::HashMap;

impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        let mut s_count = HashMap::new();
        let mut t_count = HashMap::new();

        for character in s.chars() {
            *s_count.entry(character).or_insert(0) += 1;
        }
        for character in t.chars() {
            *t_count.entry(character).or_insert(0) += 1;
        }

        s_count == t_count
    }
}

// #[test]
// fn case1() {
//     assert_eq!(vec![0, 1], Solution::two_sum(vec![2, 7, 11, 15], 9))
// }
// #[test]
// fn case2() {
//     assert_eq!(vec![1, 2], Solution::two_sum(vec![3, 2, 4], 6))
// }
// #[test]
// fn case3() {
//     assert_eq!(vec![0, 1], Solution::two_sum(vec![3, 3], 6))
// }

struct Solution;

fn main() {}
